macro_rules! deps {
    () => {
        Union!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < 'a > DoubleEndedIterator for Union < 'a > { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () } }
    };
}

impl_36!()