macro_rules! deps {
    () => {
        ParIter!();
    };
}

macro_rules! impl_154 {
    () => {
        deps!();
        impl < T > Clone for ParIter < '_ , T > { fn clone (& self) -> Self { ParIter { .. * self } } }
    };
}

impl_154!();