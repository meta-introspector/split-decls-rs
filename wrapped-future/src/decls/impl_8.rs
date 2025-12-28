macro_rules! deps {
    () => {
        AsyncActionProgressHandler_Vtbl!();
        AsyncActionProgressHandler!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        unsafe impl < TProgress : windows_core :: RuntimeType + 'static > windows_core :: Interface for AsyncActionProgressHandler < TProgress > { type Vtable = AsyncActionProgressHandler_Vtbl < TProgress > ; const IID : windows_core :: GUID = windows_core :: GUID :: from_signature (< Self as windows_core :: RuntimeType > :: SIGNATURE) ; }
    };
}

impl_8!();