// Generated macro for SingleOrMultiIndex (enum)
macro_rules! Depcrate_store_impls_dynamic_handleSingleOrMultiIndex {
() => {
// Module: crate::store_impls::dynamic::handle
// Provides: {"SingleOrMultiIndex"}
// Dependencies: {}
pub (crate) enum SingleOrMultiIndex { Single { index : Arc < gix_pack :: index :: File > , data : Option < Arc < gix_pack :: data :: File > > , } , Multi { index : Arc < gix_pack :: multi_index :: File > , data : Vec < Option < Arc < gix_pack :: data :: File > > > , } , }
};
}
