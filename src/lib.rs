use std::num::NonZeroU8;
use ptable::Element;

pub use ptable::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Isotope {
    ion: Ion,
    neutrons: Option<NonZeroU8>
}

impl Isotope {
    pub fn new(ion: Ion, neutrons: Option<NonZeroU8>) -> Isotope {
        Isotope { ion, neutrons }
    }

    pub fn from_element(element: Element, neutrons: Option<NonZeroU8>) -> Isotope {
        Self::new(Ion::from(element), neutrons)
    }

    #[inline(always)]
    pub fn get_ion(&self) -> &Ion {
        &self.ion
    }

    #[inline(always)]
    pub fn get_ion_mut(&mut self) -> &mut Ion {
        &mut self.ion
    }

    #[inline(always)]
    pub fn get_neutrons_count(&self) -> &Option<NonZeroU8> {
        &self.neutrons
    }

    #[inline(always)]
    pub fn get_neutrons_count_mut(&mut self) -> &mut Option<NonZeroU8> {
        &mut self.neutrons
    }

    #[inline(always)]
    pub fn get_element(&self) -> &Element {
        self.ion.get_element()
    }

    #[inline(always)]
    pub fn get_element_mut(&mut self) -> &mut Element {
        self.ion.get_element_mut()
    }
}

impl From<Element> for Isotope {
    fn from(i: Element) -> Isotope {
        Isotope::from_element(i, None)
    }
}

impl From<Ion> for Isotope {
    fn from(i: Ion) -> Isotope {
        Isotope::new(i, None)
    }
}

impl AsRef<Ion> for Isotope {
    fn as_ref(&self) -> &Ion {
        &self.ion
    }
}

impl AsMut<Ion> for Isotope {
    fn as_mut(&mut self) -> &mut Ion {
        &mut self.ion
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Ion {
    element: Element,
    charge: i8
}

impl Ion {
    pub fn new(element: Element, charge: i8) -> Ion {
        Ion { element, charge }
    }

    #[inline(always)]
    pub fn get_element(&self) -> &Element {
        &self.element
    }

    #[inline(always)]
    pub fn get_element_mut(&mut self) -> &mut Element {
        &mut self.element
    }

    #[inline(always)]
    pub fn set_element(&mut self, element: Element) {
        self.element = element;
    }

    #[inline(always)]
    pub fn get_charge(&self) -> &i8 {
        &self.charge
    }

    #[inline(always)]
    pub fn get_charge_mut(&mut self) -> &mut i8 {
        &mut self.charge
    }

    #[inline(always)]
    pub fn set_charge(&mut self, charge: i8) {
        self.charge = charge;
    }
}


impl From<Element> for Ion {
    fn from(e: Element) -> Ion {
        Ion::new(e, 0)
    }
}

impl AsRef<Element> for Ion {
    fn as_ref(&self) -> &Element {
        &self.element
    }
}

impl AsMut<Element> for Ion {
    fn as_mut(&mut self) -> &mut Element {
        &mut self.element
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
