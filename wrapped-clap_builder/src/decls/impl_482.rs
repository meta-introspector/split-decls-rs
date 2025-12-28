macro_rules! deps {
    () => {
        RawOccurrences!();
        RawOccurrenceValues!();
    };
}

macro_rules! impl_482 {
    () => {
        deps!();
        impl < 'a > Iterator for RawOccurrences < 'a > { type Item = RawOccurrenceValues < 'a > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_482!()