// Generated macro for impl_171 (impl)
macro_rules! Depcrate_index_setimpl_171 {
() => {
// Module: crate::index_set
// Provides: {"impl_171"}
// Dependencies: {}
impl < T , S , const N : usize > fmt :: Debug for IndexSet < T , S , N > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_set () . entries (self . iter ()) . finish () } }
};
}
