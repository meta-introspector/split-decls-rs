// Generated macro for impl_466 (impl)
macro_rules! Depcrate_scopeguardimpl_466 {
() => {
// Module: crate::scopeguard
// Provides: {"impl_466"}
// Dependencies: {}
impl < T , F > Drop for ScopeGuard < T , F > where F : FnMut (& mut T) , { # [inline] fn drop (& mut self) { (self . dropfn) (& mut self . value) ; } }
};
}
