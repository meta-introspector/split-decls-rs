macro_rules! deps {
    () => {
        AsyncOperationProgressHandler_Vtbl!();
        AsyncOperationProgressHandler!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        unsafe impl < TResult : windows_core :: RuntimeType + 'static , TProgress : windows_core :: RuntimeType + 'static , > windows_core :: Interface for AsyncOperationProgressHandler < TResult , TProgress > { type Vtable = AsyncOperationProgressHandler_Vtbl < TResult , TProgress > ; const IID : windows_core :: GUID = windows_core :: GUID :: from_signature (< Self as windows_core :: RuntimeType > :: SIGNATURE) ; }
    };
}

impl_29!()