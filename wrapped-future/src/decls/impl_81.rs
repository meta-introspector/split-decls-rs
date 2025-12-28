macro_rules! deps {
    () => {
        IAsyncOperation!();
        IAsyncOperation_Vtbl!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        unsafe impl < TResult : windows_core :: RuntimeType + 'static > windows_core :: Interface for IAsyncOperation < TResult > { type Vtable = IAsyncOperation_Vtbl < TResult > ; const IID : windows_core :: GUID = windows_core :: GUID :: from_signature (< Self as windows_core :: RuntimeType > :: SIGNATURE) ; }
    };
}

impl_81!();