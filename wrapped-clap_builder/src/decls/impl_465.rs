macro_rules! deps {
    () => {
        Occurrences!();
    };
}

macro_rules! impl_465 {
    () => {
        deps!();
        impl < T > DoubleEndedIterator for Occurrences < T > { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () } }
    };
}

impl_465!()