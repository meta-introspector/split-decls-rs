macro_rules! deps {
    () => {
        Unique!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl < T : ? Sized > Clone for Unique < T > { # [inline] fn clone (& self) -> Self { * self } }
    };
}

impl_208!()