macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! LOCAL_MACHINE {
    () => {
        deps!();
        # [doc = " The predefined `HKEY_LOCAL_MACHINE` registry key."] pub const LOCAL_MACHINE : & Key = & Key (HKEY_LOCAL_MACHINE) ;
    };
}

LOCAL_MACHINE!();