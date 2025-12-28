macro_rules! deps {
    () => {
        Interned!();
        Internable!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < T : Internable + ? Sized > Deref for Interned < T > { type Target = T ; # [inline] fn deref (& self) -> & Self :: Target { & self . arc } }
    };
}

impl_14!()