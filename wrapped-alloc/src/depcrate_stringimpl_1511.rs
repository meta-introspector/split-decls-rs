// Generated macro for impl_1511 (impl)
macro_rules! Depcrate_stringimpl_1511 {
() => {
// Module: crate::string
// Provides: {"impl_1511"}
// Dependencies: {}
# [stable (feature = "drain" , since = "1.6.0")] impl Drop for Drain < '_ > { fn drop (& mut self) { unsafe { let self_vec = (* self . string) . as_mut_vec () ; if self . start <= self . end && self . end <= self_vec . len () { self_vec . drain (self . start .. self . end) ; } } } }
};
}
