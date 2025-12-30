// Generated macro for impl_183 (impl)
macro_rules! Depcrate_linked_hash_setimpl_183 {
() => {
// Module: crate::linked_hash_set
// Provides: {"impl_183"}
// Dependencies: {}
impl < T , S > fmt :: Debug for Difference < '_ , T , S > where T : fmt :: Debug + Eq + Hash , S : BuildHasher , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
