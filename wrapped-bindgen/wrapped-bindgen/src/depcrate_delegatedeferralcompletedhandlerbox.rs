// Generated macro for DeferralCompletedHandlerBox (struct)
macro_rules! Depcrate_delegateDeferralCompletedHandlerBox {
() => {
// Module: crate::delegate
// Provides: {"DeferralCompletedHandlerBox"}
// Dependencies: {}
# [repr (C)] struct DeferralCompletedHandlerBox < F : Fn () -> windows_core :: Result < () > + Send + 'static > { vtable : * const DeferralCompletedHandler_Vtbl , invoke : F , count : windows_core :: imp :: RefCount , }
};
}
