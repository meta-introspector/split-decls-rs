macro_rules! deps {
    () => {
        HRESULT!();
    };
}

macro_rules! S_OK {
    () => {
        deps!();
        pub const S_OK : windows_core :: HRESULT = windows_core :: HRESULT (0x0_u32 as _) ;
    };
}

S_OK!()