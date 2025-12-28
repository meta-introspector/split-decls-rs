macro_rules! deps {
    () => {
        Idx!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < T > Clone for Idx < T > { fn clone (& self) -> Self { * self } }
    };
}

impl_29!()