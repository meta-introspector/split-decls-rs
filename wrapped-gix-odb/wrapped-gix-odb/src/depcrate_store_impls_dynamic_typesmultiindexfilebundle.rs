// Generated macro for MultiIndexFileBundle (struct)
macro_rules! Depcrate_store_impls_dynamic_typesMultiIndexFileBundle {
() => {
// Module: crate::store_impls::dynamic::types
// Provides: {"MultiIndexFileBundle"}
// Dependencies: {}
# [derive (Clone)] pub (crate) struct MultiIndexFileBundle { pub multi_index : OnDiskFile < Arc < gix_pack :: multi_index :: File > > , pub data : Vec < OnDiskFile < Arc < gix_pack :: data :: File > > > , }
};
}
