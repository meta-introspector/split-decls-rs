macro_rules! deps {
    () => {
        OccurrenceValues!();
        Occurrences!();
    };
}

macro_rules! impl_464 {
    () => {
        deps!();
        impl < T > Iterator for Occurrences < T > { type Item = OccurrenceValues < T > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_464!();