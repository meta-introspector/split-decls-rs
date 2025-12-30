// Generated macro for impl_103 (impl)
macro_rules! Depcrate_arrayvecimpl_103 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_103"}
// Dependencies: {}
impl < T , Data , F > Drop for ScopeExitGuard < T , Data , F > where F : FnMut (& Data , & mut T) , { fn drop (& mut self) { (self . f) (& self . data , & mut self . value) } }
};
}
