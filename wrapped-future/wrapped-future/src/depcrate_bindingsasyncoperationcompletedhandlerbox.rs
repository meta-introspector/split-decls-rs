// Generated macro for AsyncOperationCompletedHandlerBox (struct)
macro_rules! Depcrate_bindingsAsyncOperationCompletedHandlerBox {
() => {
// Module: crate::bindings
// Provides: {"AsyncOperationCompletedHandlerBox"}
// Dependencies: {}
# [repr (C)] struct AsyncOperationCompletedHandlerBox < TResult , F : Fn (windows_core :: Ref < IAsyncOperation < TResult > > , AsyncStatus) -> windows_core :: Result < () > + Send + 'static , > where TResult : windows_core :: RuntimeType + 'static , { vtable : * const AsyncOperationCompletedHandler_Vtbl < TResult > , invoke : F , count : windows_core :: imp :: RefCount , }
};
}
