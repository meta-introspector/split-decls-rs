macro_rules! deps {
    () => {
        Internable!();
        Interned!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < T : Internable > Eq for Interned < T > { }
    };
}

impl_34!();