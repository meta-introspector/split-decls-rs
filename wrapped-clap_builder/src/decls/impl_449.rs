macro_rules! deps {
    () => {
        Values!();
    };
}

macro_rules! impl_449 {
    () => {
        deps!();
        impl < T > Iterator for Values < T > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { if let Some (next) = self . iter . next () { self . len -= 1 ; Some (next) } else { None } } fn size_hint (& self) -> (usize , Option < usize >) { (self . len , Some (self . len)) } }
    };
}

impl_449!();