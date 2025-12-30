// Generated macro for impl_179 (impl)
macro_rules! Depcrate_delegateimpl_179 {
() => {
// Module: crate::delegate
// Provides: {"impl_179"}
// Dependencies: {}
impl DeferralCompletedHandler { pub fn new < F : Fn () -> windows_core :: Result < () > + Send + 'static > (invoke : F) -> Self { let com = DeferralCompletedHandlerBox { vtable : & DeferralCompletedHandlerBox :: < F > :: VTABLE , count : windows_core :: imp :: RefCount :: new (1) , invoke , } ; unsafe { core :: mem :: transmute (windows_core :: imp :: Box :: new (com)) } } pub fn Invoke (& self) -> windows_core :: Result < () > { let this = self ; unsafe { (windows_core :: Interface :: vtable (this) . Invoke) (windows_core :: Interface :: as_raw (this)) . ok () } } }
};
}
