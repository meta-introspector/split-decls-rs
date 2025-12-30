// Generated macro for is_node_local_to_unit (function)
macro_rules! Depcrate_debuginfo_utilsis_node_local_to_unit {
() => {
// Module: crate::debuginfo::utils
// Provides: {"is_node_local_to_unit"}
// Dependencies: {}
pub (crate) fn is_node_local_to_unit (cx : & CodegenCx < '_ , '_ > , def_id : DefId) -> bool { ! cx . tcx . is_reachable_non_generic (def_id) }
};
}
