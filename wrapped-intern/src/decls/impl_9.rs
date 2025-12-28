macro_rules! deps {
    () => {
        Interned!();
        Internable!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < T : Internable > Eq for Interned < T > { }
    };
}

impl_9!()