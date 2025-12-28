macro_rules! deps {
    () => {
        Indices!();
    };
}

macro_rules! impl_492 {
    () => {
        deps!();
        impl DoubleEndedIterator for Indices < '_ > { fn next_back (& mut self) -> Option < usize > { if let Some (next) = self . iter . next_back () { self . len -= 1 ; Some (next) } else { None } } }
    };
}

impl_492!();