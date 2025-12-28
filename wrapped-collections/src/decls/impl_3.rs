macro_rules! deps {
    () => {
        IIterable!();
        IIterable_Vtbl!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        unsafe impl < T : windows_core :: RuntimeType + 'static > windows_core :: Interface for IIterable < T > { type Vtable = IIterable_Vtbl < T > ; const IID : windows_core :: GUID = windows_core :: GUID :: from_signature (< Self as windows_core :: RuntimeType > :: SIGNATURE) ; }
    };
}

impl_3!()