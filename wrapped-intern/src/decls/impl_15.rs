macro_rules! deps {
    () => {
        Interned!();
        Internable!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < T : Internable + ? Sized > Clone for Interned < T > { fn clone (& self) -> Self { Self { arc : self . arc . clone () } } }
    };
}

impl_15!()