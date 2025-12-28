macro_rules! deps {
    () => {
        Deserializer!();
    };
}

macro_rules! macro_50 {
    () => {
        deps!();
        deref_erased_deserializer ! (<'de , T > Deserializer <'de > for Box < T > where T : ? Sized + Deserializer <'de >) ;
    };
}

macro_50!()