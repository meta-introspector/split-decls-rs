// Generated macro for impl_188 (impl)
macro_rules! Depcrate_linked_hash_setimpl_188 {
() => {
// Module: crate::linked_hash_set
// Provides: {"impl_188"}
// Dependencies: {}
impl < T , S > fmt :: Debug for Union < '_ , T , S > where T : fmt :: Debug + Eq + Hash , S : BuildHasher , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
