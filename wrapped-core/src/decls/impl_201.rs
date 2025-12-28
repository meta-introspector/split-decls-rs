macro_rules! deps {
    () => {
        IUnknown_Vtbl!();
        IUnknown!();
        Interface!();
        GUID!();
    };
}

macro_rules! impl_201 {
    () => {
        deps!();
        unsafe impl Interface for IUnknown { type Vtable = IUnknown_Vtbl ; const IID : GUID = GUID :: from_u128 (0x00000000_0000_0000_c000_000000000046) ; }
    };
}

impl_201!();