macro_rules! deps {
    () => {
        SendPtr!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < T > Clone for SendPtr < T > { fn clone (& self) -> Self { * self } }
    };
}

impl_22!()