macro_rules! deps {
    () => {
        Shared!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < Fut > Clone for Shared < Fut > where Fut : Future , { fn clone (& self) -> Self { Self { inner : self . inner . clone () , waker_key : NULL_WAKER_KEY } } }
    };
}

impl_103!();