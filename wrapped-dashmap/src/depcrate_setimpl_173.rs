// Generated macro for impl_173 (impl)
macro_rules! Depcrate_setimpl_173 {
() => {
// Module: crate::set
// Provides: {"impl_173"}
// Dependencies: {}
impl < K : Eq + Hash + Clone , S : Clone > Clone for DashSet < K , S > { fn clone (& self) -> Self { Self { inner : self . inner . clone () , } } fn clone_from (& mut self , source : & Self) { self . inner . clone_from (& source . inner) } }
};
}
