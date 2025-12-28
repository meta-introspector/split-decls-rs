macro_rules! deps {
    () => {
        AsyncOperationProgressHandler_Vtbl!();
        IAsyncOperationWithProgress!();
    };
}

macro_rules! AsyncOperationProgressHandlerBox {
    () => {
        deps!();
        # [repr (C)] struct AsyncOperationProgressHandlerBox < TResult , TProgress , F : Fn (windows_core :: Ref < IAsyncOperationWithProgress < TResult , TProgress > > , windows_core :: Ref < TProgress > ,) -> windows_core :: Result < () > + Send + 'static , > where TResult : windows_core :: RuntimeType + 'static , TProgress : windows_core :: RuntimeType + 'static , { vtable : * const AsyncOperationProgressHandler_Vtbl < TResult , TProgress > , invoke : F , count : windows_core :: imp :: RefCount , }
    };
}

AsyncOperationProgressHandlerBox!()