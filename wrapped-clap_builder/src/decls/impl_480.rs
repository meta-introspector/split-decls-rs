macro_rules! deps {
    () => {
        OccurrenceValuesRef!();
    };
}

macro_rules! impl_480 {
    () => {
        deps!();
        impl < 'a , T > ExactSizeIterator for OccurrenceValuesRef < 'a , T > where Self : 'a { }
    };
}

impl_480!();