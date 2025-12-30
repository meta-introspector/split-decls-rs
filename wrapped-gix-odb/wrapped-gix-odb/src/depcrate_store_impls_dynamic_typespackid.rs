// Generated macro for PackId (struct)
macro_rules! Depcrate_store_impls_dynamic_typesPackId {
() => {
// Module: crate::store_impls::dynamic::types
// Provides: {"PackId"}
// Dependencies: {}
# [doc = " A way to load and refer to a pack uniquely, namespaced by their indexing mechanism, aka multi-pack or not."] # [derive (Debug , Copy , Clone , Eq , PartialEq)] pub struct PackId { # [doc = " This is the index in the slot map at which the packs index is located."] pub (crate) index : IndexId , # [doc = " If the pack is in a multi-pack index, this additional index is the pack-index within the multi-pack index identified by `index`."] pub (crate) multipack_index : Option < gix_pack :: multi_index :: PackIndex > , }
};
}
