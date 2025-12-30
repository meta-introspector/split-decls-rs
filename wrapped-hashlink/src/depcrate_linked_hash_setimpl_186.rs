// Generated macro for impl_186 (impl)
macro_rules! Depcrate_linked_hash_setimpl_186 {
() => {
// Module: crate::linked_hash_set
// Provides: {"impl_186"}
// Dependencies: {}
impl < T , S > fmt :: Debug for SymmetricDifference < '_ , T , S > where T : fmt :: Debug + Eq + Hash , S : BuildHasher , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
