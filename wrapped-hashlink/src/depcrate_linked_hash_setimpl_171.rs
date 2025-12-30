// Generated macro for impl_171 (impl)
macro_rules! Depcrate_linked_hash_setimpl_171 {
() => {
// Module: crate::linked_hash_set
// Provides: {"impl_171"}
// Dependencies: {}
impl < K : fmt :: Debug > fmt :: Debug for Iter < '_ , K > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
