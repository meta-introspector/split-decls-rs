macro_rules! deps {
    () => {
        OccurrencesRef!();
    };
}

macro_rules! impl_474 {
    () => {
        deps!();
        impl < 'a , T > DoubleEndedIterator for OccurrencesRef < 'a , T > where Self : 'a , { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () } }
    };
}

impl_474!();