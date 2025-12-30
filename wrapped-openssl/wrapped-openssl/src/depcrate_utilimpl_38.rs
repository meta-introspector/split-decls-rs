// Generated macro for impl_38 (impl)
macro_rules! Depcrate_utilimpl_38 {
() => {
// Module: crate::util
// Provides: {"impl_38"}
// Dependencies: {}
impl < F > Drop for CallbackState < F > { fn drop (& mut self) { if let Some (panic) = self . panic . take () { panic :: resume_unwind (panic) ; } } }
};
}
