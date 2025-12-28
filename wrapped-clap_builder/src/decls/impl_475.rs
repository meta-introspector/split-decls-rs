macro_rules! deps {
    () => {
        OccurrencesRef!();
    };
}

macro_rules! impl_475 {
    () => {
        deps!();
        impl < 'a , T > ExactSizeIterator for OccurrencesRef < 'a , T > where Self : 'a { }
    };
}

impl_475!()