macro_rules! deps {
    () => {
        OccurrenceValues!();
    };
}

macro_rules! impl_470 {
    () => {
        deps!();
        impl < T > DoubleEndedIterator for OccurrenceValues < T > { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () } }
    };
}

impl_470!()