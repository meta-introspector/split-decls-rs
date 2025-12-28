macro_rules! deps {
    () => {
        IAsyncActionWithProgress_Vtbl!();
        IAsyncActionWithProgress!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        unsafe impl < TProgress : windows_core :: RuntimeType + 'static > windows_core :: Interface for IAsyncActionWithProgress < TProgress > { type Vtable = IAsyncActionWithProgress_Vtbl < TProgress > ; const IID : windows_core :: GUID = windows_core :: GUID :: from_signature (< Self as windows_core :: RuntimeType > :: SIGNATURE) ; }
    };
}

impl_60!()