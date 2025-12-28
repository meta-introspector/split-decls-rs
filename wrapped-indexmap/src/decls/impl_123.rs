macro_rules! deps {
    () => {
        ParKeys!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl < K , V > Clone for ParKeys < '_ , K , V > { fn clone (& self) -> Self { ParKeys { .. * self } } }
    };
}

impl_123!()