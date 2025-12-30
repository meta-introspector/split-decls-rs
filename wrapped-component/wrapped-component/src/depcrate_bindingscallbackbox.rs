// Generated macro for CallbackBox (struct)
macro_rules! Depcrate_bindingsCallbackBox {
() => {
// Module: crate::bindings
// Provides: {"CallbackBox"}
// Dependencies: {}
# [repr (C)] struct CallbackBox < F : Fn (i32) -> windows_core :: Result < i32 > + Send + 'static > { vtable : * const Callback_Vtbl , invoke : F , count : windows_core :: imp :: RefCount , }
};
}
