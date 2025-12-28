macro_rules! deps {
    () => {
        GUID!();
        IInspectable!();
        Interface!();
        IInspectable_Vtbl!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        unsafe impl Interface for IInspectable { type Vtable = IInspectable_Vtbl ; const IID : GUID = GUID :: from_u128 (0xaf86e2e0_b12d_4c6a_9c5a_d7aa65101e90) ; }
    };
}

impl_149!();