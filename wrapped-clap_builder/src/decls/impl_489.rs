macro_rules! deps {
    () => {
        RawOccurrenceValues!();
    };
}

macro_rules! impl_489 {
    () => {
        deps!();
        impl ExactSizeIterator for RawOccurrenceValues < '_ > { }
    };
}

impl_489!();