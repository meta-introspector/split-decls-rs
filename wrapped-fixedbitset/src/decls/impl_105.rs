macro_rules! deps {
    () => {
        Union!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl < 'a > DoubleEndedIterator for Union < 'a > { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () } }
    };
}

impl_105!();