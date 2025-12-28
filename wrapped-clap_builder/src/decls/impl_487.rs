macro_rules! deps {
    () => {
        OsStr!();
        RawOccurrenceValues!();
    };
}

macro_rules! impl_487 {
    () => {
        deps!();
        impl < 'a > Iterator for RawOccurrenceValues < 'a > where Self : 'a , { type Item = & 'a OsStr ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_487!();