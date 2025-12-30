// Generated macro for impl_23 (impl)
macro_rules! Depcrate_store_impls_dynamic_findimpl_23 {
() => {
// Module: crate::store_impls::dynamic::find
// Provides: {"impl_23"}
// Dependencies: {}
impl < S > gix_object :: Find for super :: Handle < S > where S : Deref < Target = super :: Store > + Clone , Self : gix_pack :: Find , { fn try_find < 'a > (& self , id : & gix_hash :: oid , buffer : & 'a mut Vec < u8 > ,) -> Result < Option < gix_object :: Data < 'a > > , gix_object :: find :: Error > { gix_pack :: Find :: try_find (self , id , buffer) . map (| t | t . map (| t | t . 0)) } }
};
}
