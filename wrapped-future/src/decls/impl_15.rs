macro_rules! deps {
    () => {
        AsyncActionWithProgressCompletedHandler!();
        AsyncActionWithProgressCompletedHandler_Vtbl!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        unsafe impl < TProgress : windows_core :: RuntimeType + 'static > windows_core :: Interface for AsyncActionWithProgressCompletedHandler < TProgress > { type Vtable = AsyncActionWithProgressCompletedHandler_Vtbl < TProgress > ; const IID : windows_core :: GUID = windows_core :: GUID :: from_signature (< Self as windows_core :: RuntimeType > :: SIGNATURE) ; }
    };
}

impl_15!();