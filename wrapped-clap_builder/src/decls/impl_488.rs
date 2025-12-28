macro_rules! deps {
    () => {
        RawOccurrenceValues!();
    };
}

macro_rules! impl_488 {
    () => {
        deps!();
        impl < 'a > DoubleEndedIterator for RawOccurrenceValues < 'a > where Self : 'a , { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () } }
    };
}

impl_488!();