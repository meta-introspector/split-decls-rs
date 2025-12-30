// Generated macro for impl_465 (impl)
macro_rules! Depcrate_scopeguardimpl_465 {
() => {
// Module: crate::scopeguard
// Provides: {"impl_465"}
// Dependencies: {}
impl < T , F > DerefMut for ScopeGuard < T , F > where F : FnMut (& mut T) , { # [inline] fn deref_mut (& mut self) -> & mut T { & mut self . value } }
};
}
