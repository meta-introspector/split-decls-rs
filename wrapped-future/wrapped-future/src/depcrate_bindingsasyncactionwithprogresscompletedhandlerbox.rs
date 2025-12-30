// Generated macro for AsyncActionWithProgressCompletedHandlerBox (struct)
macro_rules! Depcrate_bindingsAsyncActionWithProgressCompletedHandlerBox {
() => {
// Module: crate::bindings
// Provides: {"AsyncActionWithProgressCompletedHandlerBox"}
// Dependencies: {}
# [repr (C)] struct AsyncActionWithProgressCompletedHandlerBox < TProgress , F : Fn (windows_core :: Ref < IAsyncActionWithProgress < TProgress > > , AsyncStatus ,) -> windows_core :: Result < () > + Send + 'static , > where TProgress : windows_core :: RuntimeType + 'static , { vtable : * const AsyncActionWithProgressCompletedHandler_Vtbl < TProgress > , invoke : F , count : windows_core :: imp :: RefCount , }
};
}
