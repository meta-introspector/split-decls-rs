macro_rules! deps {
    () => {
        RawOccurrences!();
    };
}

macro_rules! impl_484 {
    () => {
        deps!();
        impl ExactSizeIterator for RawOccurrences < '_ > { }
    };
}

impl_484!()