macro_rules! deps {
    () => {
        ReadBuffer!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < BS : ArraySize > Clone for ReadBuffer < BS > { # [inline] fn clone (& self) -> Self { let buffer = self . buffer . clone () ; Self { buffer } } }
    };
}

impl_3!();