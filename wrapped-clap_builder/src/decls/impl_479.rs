macro_rules! deps {
    () => {
        OccurrenceValuesRef!();
    };
}

macro_rules! impl_479 {
    () => {
        deps!();
        impl < 'a , T > DoubleEndedIterator for OccurrenceValuesRef < 'a , T > where Self : 'a , { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () } }
    };
}

impl_479!();