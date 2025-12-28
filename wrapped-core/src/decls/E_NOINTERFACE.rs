macro_rules! deps {
    () => {
        HRESULT!();
    };
}

macro_rules! E_NOINTERFACE {
    () => {
        deps!();
        pub const E_NOINTERFACE : windows_core :: HRESULT = windows_core :: HRESULT (0x80004002_u32 as _) ;
    };
}

E_NOINTERFACE!();