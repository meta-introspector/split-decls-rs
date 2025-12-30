// Generated macro for AsyncActionProgressHandlerBox (struct)
macro_rules! Depcrate_bindingsAsyncActionProgressHandlerBox {
() => {
// Module: crate::bindings
// Provides: {"AsyncActionProgressHandlerBox"}
// Dependencies: {}
# [repr (C)] struct AsyncActionProgressHandlerBox < TProgress , F : Fn (windows_core :: Ref < IAsyncActionWithProgress < TProgress > > , windows_core :: Ref < TProgress > ,) -> windows_core :: Result < () > + Send + 'static , > where TProgress : windows_core :: RuntimeType + 'static , { vtable : * const AsyncActionProgressHandler_Vtbl < TProgress > , invoke : F , count : windows_core :: imp :: RefCount , }
};
}
