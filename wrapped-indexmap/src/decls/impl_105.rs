macro_rules! deps {
    () => {
        ParIter!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl < K , V > Clone for ParIter < '_ , K , V > { fn clone (& self) -> Self { ParIter { .. * self } } }
    };
}

impl_105!()