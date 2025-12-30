// Generated macro for impl_543 (impl)
macro_rules! Depcrate_setimpl_543 {
() => {
// Module: crate::set
// Provides: {"impl_543"}
// Dependencies: {}
impl < T , S , A > fmt :: Debug for Union < '_ , T , S , A > where T : fmt :: Debug + Eq + Hash , S : BuildHasher , A : Allocator , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
