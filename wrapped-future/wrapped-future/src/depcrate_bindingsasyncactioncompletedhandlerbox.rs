// Generated macro for AsyncActionCompletedHandlerBox (struct)
macro_rules! Depcrate_bindingsAsyncActionCompletedHandlerBox {
() => {
// Module: crate::bindings
// Provides: {"AsyncActionCompletedHandlerBox"}
// Dependencies: {}
# [repr (C)] struct AsyncActionCompletedHandlerBox < F : Fn (windows_core :: Ref < IAsyncAction > , AsyncStatus) -> windows_core :: Result < () > + Send + 'static , > { vtable : * const AsyncActionCompletedHandler_Vtbl , invoke : F , count : windows_core :: imp :: RefCount , }
};
}
