macro_rules! deps {
    () => {
        GUID!();
    };
}

macro_rules! IID_IUnknown {
    () => {
        deps!();
        pub const IID_IUnknown : GUID = GUID :: from_u128 (0x00000000_0000_0000_c000_000000000046) ;
    };
}

IID_IUnknown!()