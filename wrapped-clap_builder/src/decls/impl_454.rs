macro_rules! deps {
    () => {
        ValuesRef!();
    };
}

macro_rules! impl_454 {
    () => {
        deps!();
        impl < 'a , T : 'a > Iterator for ValuesRef < 'a , T > { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { if let Some (next) = self . iter . next () { self . len -= 1 ; Some (next) } else { None } } fn size_hint (& self) -> (usize , Option < usize >) { (self . len , Some (self . len)) } }
    };
}

impl_454!();