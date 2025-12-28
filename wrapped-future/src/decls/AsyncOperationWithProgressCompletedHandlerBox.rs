macro_rules! deps {
    () => {
        AsyncStatus!();
        AsyncOperationWithProgressCompletedHandler_Vtbl!();
        IAsyncOperationWithProgress!();
    };
}

macro_rules! AsyncOperationWithProgressCompletedHandlerBox {
    () => {
        deps!();
        # [repr (C)] struct AsyncOperationWithProgressCompletedHandlerBox < TResult , TProgress , F : Fn (windows_core :: Ref < IAsyncOperationWithProgress < TResult , TProgress > > , AsyncStatus ,) -> windows_core :: Result < () > + Send + 'static , > where TResult : windows_core :: RuntimeType + 'static , TProgress : windows_core :: RuntimeType + 'static , { vtable : * const AsyncOperationWithProgressCompletedHandler_Vtbl < TResult , TProgress > , invoke : F , count : windows_core :: imp :: RefCount , }
    };
}

AsyncOperationWithProgressCompletedHandlerBox!();