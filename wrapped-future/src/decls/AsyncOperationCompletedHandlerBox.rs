macro_rules! deps {
    () => {
        AsyncOperationCompletedHandler_Vtbl!();
        AsyncStatus!();
        IAsyncOperation!();
    };
}

macro_rules! AsyncOperationCompletedHandlerBox {
    () => {
        deps!();
        # [repr (C)] struct AsyncOperationCompletedHandlerBox < TResult , F : Fn (windows_core :: Ref < IAsyncOperation < TResult > > , AsyncStatus) -> windows_core :: Result < () > + Send + 'static , > where TResult : windows_core :: RuntimeType + 'static , { vtable : * const AsyncOperationCompletedHandler_Vtbl < TResult > , invoke : F , count : windows_core :: imp :: RefCount , }
    };
}

AsyncOperationCompletedHandlerBox!()