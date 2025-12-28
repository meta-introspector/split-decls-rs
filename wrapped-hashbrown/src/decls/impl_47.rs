macro_rules! deps {
    () => {
        Bucket!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < T > Clone for Bucket < T > { # [inline] fn clone (& self) -> Self { Self { ptr : self . ptr } } }
    };
}

impl_47!();