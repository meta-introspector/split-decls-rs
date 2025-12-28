macro_rules! deps {
    () => {
        OccurrenceValuesRef!();
    };
}

macro_rules! impl_478 {
    () => {
        deps!();
        impl < 'a , T > Iterator for OccurrenceValuesRef < 'a , T > where Self : 'a , { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_478!()