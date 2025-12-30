// Generated macro for impl_149 (impl)
macro_rules! Depcrate_linked_hash_setimpl_149 {
() => {
// Module: crate::linked_hash_set
// Provides: {"impl_149"}
// Dependencies: {}
impl < T , S > fmt :: Debug for LinkedHashSet < T , S > where T : fmt :: Debug , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_set () . entries (self . iter ()) . finish () } }
};
}
