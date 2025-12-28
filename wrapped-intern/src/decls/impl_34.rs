macro_rules! deps {
    () => {
        Interned!();
        Internable!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < T : Internable > Eq for Interned < T > { }
    };
}

impl_34!()