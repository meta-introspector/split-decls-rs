macro_rules! deps {
    () => {
        MappedRef!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash , T : ? Sized > Deref for MappedRef < 'a , K , T > { type Target = T ; fn deref (& self) -> & T { self . value () } }
    };
}

impl_73!();