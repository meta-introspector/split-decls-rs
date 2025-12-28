macro_rules! deps {
    () => {
        IAsyncOperationWithProgress_Vtbl!();
        IAsyncOperationWithProgress!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        unsafe impl < TResult : windows_core :: RuntimeType + 'static , TProgress : windows_core :: RuntimeType + 'static , > windows_core :: Interface for IAsyncOperationWithProgress < TResult , TProgress > { type Vtable = IAsyncOperationWithProgress_Vtbl < TResult , TProgress > ; const IID : windows_core :: GUID = windows_core :: GUID :: from_signature (< Self as windows_core :: RuntimeType > :: SIGNATURE) ; }
    };
}

impl_94!();