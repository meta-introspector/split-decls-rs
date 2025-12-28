macro_rules! deps {
    () => {
        IVector!();
        IVector_Vtbl!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        unsafe impl < T : windows_core :: RuntimeType + 'static > windows_core :: Interface for IVector < T > { type Vtable = IVector_Vtbl < T > ; const IID : windows_core :: GUID = windows_core :: GUID :: from_signature (< Self as windows_core :: RuntimeType > :: SIGNATURE) ; }
    };
}

impl_62!();