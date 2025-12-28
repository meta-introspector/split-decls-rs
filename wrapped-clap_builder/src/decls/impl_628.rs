macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_628 {
    () => {
        deps!();
        impl < K , V > ExactSizeIterator for IterMut < '_ , K , V > { }
    };
}

impl_628!();