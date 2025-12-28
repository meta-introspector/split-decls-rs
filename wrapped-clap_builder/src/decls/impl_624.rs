macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_624 {
    () => {
        deps!();
        impl < K , V > ExactSizeIterator for Iter < '_ , K , V > { }
    };
}

impl_624!();