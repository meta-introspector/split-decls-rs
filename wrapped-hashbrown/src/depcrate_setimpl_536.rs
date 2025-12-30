// Generated macro for impl_536 (impl)
macro_rules! Depcrate_setimpl_536 {
() => {
// Module: crate::set
// Provides: {"impl_536"}
// Dependencies: {}
impl < T , S , A > fmt :: Debug for Difference < '_ , T , S , A > where T : fmt :: Debug + Eq + Hash , S : BuildHasher , A : Allocator , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
