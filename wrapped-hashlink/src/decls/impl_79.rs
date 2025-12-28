macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < K , V > ExactSizeIterator for Iter < '_ , K , V > { }
    };
}

impl_79!();