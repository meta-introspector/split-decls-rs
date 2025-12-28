macro_rules! deps {
    () => {
        IdsRef!();
        Id!();
    };
}

macro_rules! impl_445 {
    () => {
        deps!();
        impl < 'a > Iterator for IdsRef < 'a > { type Item = & 'a Id ; fn next (& mut self) -> Option < & 'a Id > { self . iter . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_445!();