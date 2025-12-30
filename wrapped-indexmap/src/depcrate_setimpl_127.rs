// Generated macro for impl_127 (impl)
macro_rules! Depcrate_setimpl_127 {
() => {
// Module: crate::set
// Provides: {"impl_127"}
// Dependencies: {}
impl < T , S > Clone for IndexSet < T , S > where T : Clone , S : Clone , { fn clone (& self) -> Self { IndexSet { map : self . map . clone () , } } fn clone_from (& mut self , other : & Self) { self . map . clone_from (& other . map) ; } }
};
}
