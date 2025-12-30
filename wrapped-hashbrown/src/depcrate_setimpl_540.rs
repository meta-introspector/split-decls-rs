// Generated macro for impl_540 (impl)
macro_rules! Depcrate_setimpl_540 {
() => {
// Module: crate::set
// Provides: {"impl_540"}
// Dependencies: {}
impl < T , S , A > fmt :: Debug for SymmetricDifference < '_ , T , S , A > where T : fmt :: Debug + Eq + Hash , S : BuildHasher , A : Allocator , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
