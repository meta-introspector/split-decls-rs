macro_rules! deps {
    () => {
        AsyncOperationCompletedHandler_Vtbl!();
        AsyncOperationCompletedHandler!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        unsafe impl < TResult : windows_core :: RuntimeType + 'static > windows_core :: Interface for AsyncOperationCompletedHandler < TResult > { type Vtable = AsyncOperationCompletedHandler_Vtbl < TResult > ; const IID : windows_core :: GUID = windows_core :: GUID :: from_signature (< Self as windows_core :: RuntimeType > :: SIGNATURE) ; }
    };
}

impl_22!()