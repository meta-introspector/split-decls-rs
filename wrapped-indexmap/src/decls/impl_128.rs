macro_rules! deps {
    () => {
        ParValues!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl < K , V > Clone for ParValues < '_ , K , V > { fn clone (& self) -> Self { ParValues { .. * self } } }
    };
}

impl_128!()