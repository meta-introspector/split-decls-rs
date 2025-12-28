macro_rules! deps {
    () => {
        AsyncOperationWithProgressCompletedHandler_Vtbl!();
        AsyncOperationWithProgressCompletedHandler!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        unsafe impl < TResult : windows_core :: RuntimeType + 'static , TProgress : windows_core :: RuntimeType + 'static , > windows_core :: Interface for AsyncOperationWithProgressCompletedHandler < TResult , TProgress > { type Vtable = AsyncOperationWithProgressCompletedHandler_Vtbl < TResult , TProgress > ; const IID : windows_core :: GUID = windows_core :: GUID :: from_signature (< Self as windows_core :: RuntimeType > :: SIGNATURE) ; }
    };
}

impl_36!()