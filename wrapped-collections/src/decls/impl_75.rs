macro_rules! deps {
    () => {
        IVectorView_Vtbl!();
        IVectorView!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        unsafe impl < T : windows_core :: RuntimeType + 'static > windows_core :: Interface for IVectorView < T > { type Vtable = IVectorView_Vtbl < T > ; const IID : windows_core :: GUID = windows_core :: GUID :: from_signature (< Self as windows_core :: RuntimeType > :: SIGNATURE) ; }
    };
}

impl_75!()