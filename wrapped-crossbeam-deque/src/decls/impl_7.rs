macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < T > Clone for Buffer < T > { fn clone (& self) -> Self { * self } }
    };
}

impl_7!()