// Generated macro for impl_365 (impl)
macro_rules! Depcrate_vecimpl_365 {
() => {
// Module: crate::vec
// Provides: {"impl_365"}
// Dependencies: {}
impl < T , LenT : LenType , const N : usize > ExactSizeIterator for IntoIter < T , N , LenT > { fn len (& self) -> usize { (self . vec . len - self . next) . into_usize () } }
};
}
