// Generated macro for impl_240 (impl)
macro_rules! Depcrate_collections_stringimpl_240 {
() => {
// Module: crate::collections::string
// Provides: {"impl_240"}
// Dependencies: {}
impl < 'a , 'bump > Drop for Drain < 'a , 'bump > { fn drop (& mut self) { unsafe { let self_vec = (* self . string) . as_mut_vec () ; if self . start <= self . end && self . end <= self_vec . len () { self_vec . drain (self . start .. self . end) ; } } } }
};
}
