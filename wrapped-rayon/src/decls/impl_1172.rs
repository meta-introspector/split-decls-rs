macro_rules! deps {
    () => {
        RChunks!();
    };
}

macro_rules! impl_1172 {
    () => {
        deps!();
        impl < T > Clone for RChunks < '_ , T > { fn clone (& self) -> Self { RChunks { .. * self } } }
    };
}

impl_1172!()