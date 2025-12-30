// Generated macro for impl_167 (impl)
macro_rules! Depcrate_strategy_hybridimpl_167 {
() => {
// Module: crate::strategy::hybrid
// Provides: {"impl_167"}
// Dependencies: {}
impl < T : RefCnt > Drop for HybridProtection < T > { # [inline] fn drop (& mut self) { match self . debt . take () { None => () , Some (debt) => { let ptr = T :: as_ptr (& self . ptr) ; if debt . pay :: < T > (ptr) { return ; } } } unsafe { ManuallyDrop :: drop (& mut self . ptr) } ; } }
};
}
