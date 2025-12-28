macro_rules! deps {
    () => {
        HRESULT!();
    };
}

macro_rules! REGDB_E_CLASSNOTREG {
    () => {
        deps!();
        pub const REGDB_E_CLASSNOTREG : windows_core :: HRESULT = windows_core :: HRESULT (0x80040154_u32 as _) ;
    };
}

REGDB_E_CLASSNOTREG!();