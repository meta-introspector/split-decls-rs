macro_rules! deps {
    () => {
        Deserializer!();
    };
}

macro_rules! macro_49 {
    () => {
        deps!();
        deref_erased_deserializer ! (<'de , T > Deserializer <'de > for & mut T where T : ? Sized + Deserializer <'de >) ;
    };
}

macro_49!()