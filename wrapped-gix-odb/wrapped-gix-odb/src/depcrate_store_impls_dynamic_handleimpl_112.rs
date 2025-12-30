// Generated macro for impl_112 (impl)
macro_rules! Depcrate_store_impls_dynamic_handleimpl_112 {
() => {
// Module: crate::store_impls::dynamic::handle
// Provides: {"impl_112"}
// Dependencies: {}
impl < S > Drop for super :: Handle < S > where S : Deref < Target = super :: Store > + Clone , { fn drop (& mut self) { if let Some (token) = self . token . take () { self . store . remove_handle (token) ; } } }
};
}
