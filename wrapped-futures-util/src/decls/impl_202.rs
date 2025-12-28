macro_rules! deps {
    () => {
        AlwaysReady!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        impl < T , F : Fn () -> T + Clone > Clone for AlwaysReady < T , F > { fn clone (& self) -> Self { Self (self . 0 . clone ()) } }
    };
}

impl_202!();