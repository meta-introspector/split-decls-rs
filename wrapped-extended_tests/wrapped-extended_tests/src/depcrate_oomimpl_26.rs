// Generated macro for impl_26 (impl)
macro_rules! Depcrate_oomimpl_26 {
() => {
// Module: crate::oom
// Provides: {"impl_26"}
// Dependencies: {}
impl < F : FnOnce () > ExitGuard < F > { # [inline] const fn new (drop_callback : F) -> Self { Self { drop_callback : Some (drop_callback) , } } }
};
}
