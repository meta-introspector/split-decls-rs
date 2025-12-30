// Generated macro for impl_487 (impl)
macro_rules! Depcrate_setimpl_487 {
() => {
// Module: crate::set
// Provides: {"impl_487"}
// Dependencies: {}
impl < T , S , A > fmt :: Debug for HashSet < T , S , A > where T : fmt :: Debug , A : Allocator , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_set () . entries (self . iter ()) . finish () } }
};
}
