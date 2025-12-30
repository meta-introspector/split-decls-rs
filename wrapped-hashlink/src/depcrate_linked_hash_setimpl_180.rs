// Generated macro for impl_180 (impl)
macro_rules! Depcrate_linked_hash_setimpl_180 {
() => {
// Module: crate::linked_hash_set
// Provides: {"impl_180"}
// Dependencies: {}
impl < T , S > fmt :: Debug for Intersection < '_ , T , S > where T : fmt :: Debug + Eq + Hash , S : BuildHasher , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
