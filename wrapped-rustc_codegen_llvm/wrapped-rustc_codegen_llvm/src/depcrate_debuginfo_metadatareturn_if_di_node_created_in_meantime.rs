// Generated macro for return_if_di_node_created_in_meantime (macro)
macro_rules! Depcrate_debuginfo_metadatareturn_if_di_node_created_in_meantime {
() => {
// Module: crate::debuginfo::metadata
// Provides: {"return_if_di_node_created_in_meantime"}
// Dependencies: {}
# [doc = " Returns from the enclosing function if the type debuginfo node with the given"] # [doc = " unique ID can be found in the type map."] macro_rules ! return_if_di_node_created_in_meantime { ($ cx : expr , $ unique_type_id : expr) => { if let Some (di_node) = debug_context ($ cx) . type_map . di_node_for_unique_id ($ unique_type_id) { return DINodeCreationResult :: new (di_node , true) ; } } ; }
};
}
