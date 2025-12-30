// Generated macro for impl_170 (impl)
macro_rules! Depcrate_index_setimpl_170 {
() => {
// Module: crate::index_set
// Provides: {"impl_170"}
// Dependencies: {}
impl < T , S , const N : usize > Clone for IndexSet < T , S , N > where T : Clone , S : Clone , { fn clone (& self) -> Self { Self { map : self . map . clone () , } } }
};
}
