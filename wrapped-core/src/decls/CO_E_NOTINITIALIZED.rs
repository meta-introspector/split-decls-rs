macro_rules! deps {
    () => {
        HRESULT!();
    };
}

macro_rules! CO_E_NOTINITIALIZED {
    () => {
        deps!();
        pub const CO_E_NOTINITIALIZED : windows_core :: HRESULT = windows_core :: HRESULT (0x800401F0_u32 as _) ;
    };
}

CO_E_NOTINITIALIZED!();