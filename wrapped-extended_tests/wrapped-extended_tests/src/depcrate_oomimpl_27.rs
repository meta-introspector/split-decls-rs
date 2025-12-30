// Generated macro for impl_27 (impl)
macro_rules! Depcrate_oomimpl_27 {
() => {
// Module: crate::oom
// Provides: {"impl_27"}
// Dependencies: {}
impl < F : FnOnce () > Drop for ExitGuard < F > { fn drop (& mut self) { if let Some (f) = self . drop_callback . take () { f () ; } } }
};
}
