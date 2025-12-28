macro_rules! deps {
    () => {
        Idx!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < T > Clone for Idx < T > { fn clone (& self) -> Self { * self } }
    };
}

impl_10!()