// Generated macro for impl_150 (impl)
macro_rules! Depcrate_collections_vec_dequeimpl_150 {
() => {
// Module: crate::collections::vec_deque
// Provides: {"impl_150"}
// Dependencies: {}
impl < T > Drop for Drain < '_ , T > { fn drop (& mut self) { if self . deque . len () != self . orig_len - self . range . len () { assert_eq ! (self . deque . len () , self . orig_len) ; self . deque . drain (self . range . clone ()) ; } } }
};
}
