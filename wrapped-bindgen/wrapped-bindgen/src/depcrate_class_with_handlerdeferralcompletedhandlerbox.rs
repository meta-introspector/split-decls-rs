// Generated macro for DeferralCompletedHandlerBox (struct)
macro_rules! Depcrate_class_with_handlerDeferralCompletedHandlerBox {
() => {
// Module: crate::class_with_handler
// Provides: {"DeferralCompletedHandlerBox"}
// Dependencies: {}
# [repr (C)] struct DeferralCompletedHandlerBox < F : Fn () -> windows_core :: Result < () > + Send + 'static > { vtable : * const DeferralCompletedHandler_Vtbl , invoke : F , count : windows_core :: imp :: RefCount , }
};
}
