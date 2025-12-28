macro_rules! deps {
    () => {
        MappedRef!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash , T : ? Sized + AsRef < TDeref > , TDeref : ? Sized > AsRef < TDeref > for MappedRef < 'a , K , T > { fn as_ref (& self) -> & TDeref { self . value () . as_ref () } }
    };
}

impl_75!()