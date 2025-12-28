macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl < K , V > ExactSizeIterator for IterMut < '_ , K , V > { }
    };
}

impl_80!()