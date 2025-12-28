macro_rules! deps {
    () => {
        Id!();
        IdsRef!();
    };
}

macro_rules! impl_446 {
    () => {
        deps!();
        impl < 'a > DoubleEndedIterator for IdsRef < 'a > { fn next_back (& mut self) -> Option < & 'a Id > { self . iter . next_back () } }
    };
}

impl_446!();