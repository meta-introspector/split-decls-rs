// Generated macro for impl_464 (impl)
macro_rules! Depcrate_scopeguardimpl_464 {
() => {
// Module: crate::scopeguard
// Provides: {"impl_464"}
// Dependencies: {}
impl < T , F > Deref for ScopeGuard < T , F > where F : FnMut (& mut T) , { type Target = T ; # [inline] fn deref (& self) -> & T { & self . value } }
};
}
