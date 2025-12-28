macro_rules! deps {
    () => {
        OccurrenceValues!();
    };
}

macro_rules! impl_469 {
    () => {
        deps!();
        impl < T > Iterator for OccurrenceValues < T > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_469!()