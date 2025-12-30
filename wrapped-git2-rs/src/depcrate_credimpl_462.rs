// Generated macro for impl_462 (impl)
macro_rules! Depcrate_credimpl_462 {
() => {
// Module: crate::cred
// Provides: {"impl_462"}
// Dependencies: {}
impl Drop for Cred { fn drop (& mut self) { if ! self . raw . is_null () { unsafe { if let Some (f) = (* self . raw) . free { f (self . raw) } } } } }
};
}
