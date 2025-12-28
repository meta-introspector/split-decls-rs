macro_rules! deps {
    () => {
        Keys!();
    };
}

macro_rules! impl_314 {
    () => {
        deps!();
        impl < K , V > FusedIterator for Keys < '_ , K , V > { }
    };
}

impl_314!()