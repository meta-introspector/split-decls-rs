// Generated macro for impl_25 (impl)
macro_rules! Depcrate_store_impls_dynamic_findimpl_25 {
() => {
// Module: crate::store_impls::dynamic::find
// Provides: {"impl_25"}
// Dependencies: {}
impl < S > gix_object :: Exists for super :: Handle < S > where S : Deref < Target = super :: Store > + Clone , Self : gix_pack :: Find , { fn exists (& self , id : & gix_hash :: oid) -> bool { gix_pack :: Find :: contains (self , id) } }
};
}
