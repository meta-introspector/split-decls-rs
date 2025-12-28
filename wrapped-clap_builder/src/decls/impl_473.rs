macro_rules! deps {
    () => {
        OccurrencesRef!();
        OccurrenceValuesRef!();
    };
}

macro_rules! impl_473 {
    () => {
        deps!();
        impl < 'a , T > Iterator for OccurrencesRef < 'a , T > where Self : 'a , { type Item = OccurrenceValuesRef < 'a , T > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_473!();