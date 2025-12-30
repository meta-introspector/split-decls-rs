// Generated macro for impl_531 (impl)
macro_rules! Depcrate_setimpl_531 {
() => {
// Module: crate::set
// Provides: {"impl_531"}
// Dependencies: {}
impl < T , S , A > fmt :: Debug for Intersection < '_ , T , S , A > where T : fmt :: Debug + Eq + Hash , S : BuildHasher , A : Allocator , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
