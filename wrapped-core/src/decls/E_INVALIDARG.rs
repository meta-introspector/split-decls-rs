macro_rules! deps {
    () => {
        HRESULT!();
    };
}

macro_rules! E_INVALIDARG {
    () => {
        deps!();
        pub const E_INVALIDARG : windows_core :: HRESULT = windows_core :: HRESULT (0x80070057_u32 as _) ;
    };
}

E_INVALIDARG!();