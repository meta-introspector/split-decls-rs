// Generated macro for impl_567 (impl)
macro_rules! Depcrate_tableimpl_567 {
() => {
// Module: crate::table
// Provides: {"impl_567"}
// Dependencies: {}
impl < T , A > fmt :: Debug for HashTable < T , A > where T : fmt :: Debug , A : Allocator , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_set () . entries (self . iter ()) . finish () } }
};
}
