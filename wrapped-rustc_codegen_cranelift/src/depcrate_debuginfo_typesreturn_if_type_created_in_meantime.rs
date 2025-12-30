// Generated macro for return_if_type_created_in_meantime (macro)
macro_rules! Depcrate_debuginfo_typesreturn_if_type_created_in_meantime {
() => {
// Module: crate::debuginfo::types
// Provides: {"return_if_type_created_in_meantime"}
// Dependencies: {}
# [doc = " Returns from the enclosing function if the type debuginfo node with the given"] # [doc = " unique ID can be found in the type map."] macro_rules ! return_if_type_created_in_meantime { ($ type_dbg : expr , $ ty : expr) => { if let Some (& type_id) = $ type_dbg . type_map . get (&$ ty) { return type_id ; } } ; }
};
}
