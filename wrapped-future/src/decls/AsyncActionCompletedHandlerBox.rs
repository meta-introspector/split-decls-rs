macro_rules! deps {
    () => {
        AsyncActionCompletedHandler_Vtbl!();
        AsyncStatus!();
    };
}

macro_rules! AsyncActionCompletedHandlerBox {
    () => {
        deps!();
        # [repr (C)] struct AsyncActionCompletedHandlerBox < F : Fn (windows_core :: Ref < IAsyncAction > , AsyncStatus) -> windows_core :: Result < () > + Send + 'static , > { vtable : * const AsyncActionCompletedHandler_Vtbl , invoke : F , count : windows_core :: imp :: RefCount , }
    };
}

AsyncActionCompletedHandlerBox!();