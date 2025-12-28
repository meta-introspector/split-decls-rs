macro_rules! deps {
    () => {
        Stealer!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < T > Clone for Stealer < T > { fn clone (& self) -> Self { Self { inner : self . inner . clone () , flavor : self . flavor , } } }
    };
}

impl_20!();