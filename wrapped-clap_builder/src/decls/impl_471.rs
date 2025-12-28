macro_rules! deps {
    () => {
        OccurrenceValues!();
    };
}

macro_rules! impl_471 {
    () => {
        deps!();
        impl < T > ExactSizeIterator for OccurrenceValues < T > { }
    };
}

impl_471!();