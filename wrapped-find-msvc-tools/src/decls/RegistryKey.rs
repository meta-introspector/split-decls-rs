macro_rules! deps {
    () => {
        Repr!();
    };
}

macro_rules! RegistryKey {
    () => {
        deps!();
        # [doc = " Must never be `HKEY_PERFORMANCE_DATA`."] pub (crate) struct RegistryKey (Repr) ;
    };
}

RegistryKey!();