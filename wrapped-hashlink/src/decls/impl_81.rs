macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < K , V > ExactSizeIterator for IntoIter < K , V > { }
    };
}

impl_81!()