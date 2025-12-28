macro_rules! deps {
    () => {
        Internable!();
        Interned!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < T : Internable + ? Sized > Clone for Interned < T > { fn clone (& self) -> Self { Self { arc : self . arc . clone () } } }
    };
}

impl_40!();