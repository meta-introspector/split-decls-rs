macro_rules! deps {
    () => {
        SymmetricDifference!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl < 'a > DoubleEndedIterator for SymmetricDifference < 'a > { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () } }
    };
}

impl_97!()