macro_rules! deps {
    () => {
        RawOccurrences!();
    };
}

macro_rules! impl_483 {
    () => {
        deps!();
        impl DoubleEndedIterator for RawOccurrences < '_ > { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () } }
    };
}

impl_483!()