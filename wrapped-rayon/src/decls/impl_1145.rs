macro_rules! deps {
    () => {
        Chunks!();
    };
}

macro_rules! impl_1145 {
    () => {
        deps!();
        impl < T > Clone for Chunks < '_ , T > { fn clone (& self) -> Self { Chunks { .. * self } } }
    };
}

impl_1145!()