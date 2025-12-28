macro_rules! deps {
    () => {
        HRESULT!();
    };
}

macro_rules! E_POINTER {
    () => {
        deps!();
        pub const E_POINTER : windows_core :: HRESULT = windows_core :: HRESULT (0x80004003_u32 as _) ;
    };
}

E_POINTER!()