macro_rules! deps {
    () => {
        Internable!();
        Interned!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < T : Internable + ? Sized > Deref for Interned < T > { type Target = T ; # [inline] fn deref (& self) -> & Self :: Target { & self . arc } }
    };
}

impl_39!();