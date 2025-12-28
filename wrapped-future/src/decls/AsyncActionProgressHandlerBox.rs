macro_rules! deps {
    () => {
        AsyncActionProgressHandler_Vtbl!();
        IAsyncActionWithProgress!();
    };
}

macro_rules! AsyncActionProgressHandlerBox {
    () => {
        deps!();
        # [repr (C)] struct AsyncActionProgressHandlerBox < TProgress , F : Fn (windows_core :: Ref < IAsyncActionWithProgress < TProgress > > , windows_core :: Ref < TProgress > ,) -> windows_core :: Result < () > + Send + 'static , > where TProgress : windows_core :: RuntimeType + 'static , { vtable : * const AsyncActionProgressHandler_Vtbl < TProgress > , invoke : F , count : windows_core :: imp :: RefCount , }
    };
}

AsyncActionProgressHandlerBox!()