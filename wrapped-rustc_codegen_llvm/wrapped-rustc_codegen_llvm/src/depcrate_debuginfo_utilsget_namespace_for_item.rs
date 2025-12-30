// Generated macro for get_namespace_for_item (function)
macro_rules! Depcrate_debuginfo_utilsget_namespace_for_item {
() => {
// Module: crate::debuginfo::utils
// Provides: {"get_namespace_for_item"}
// Dependencies: {}
pub (crate) fn get_namespace_for_item < 'll > (cx : & CodegenCx < 'll , '_ > , def_id : DefId) -> & 'll DIScope { item_namespace (cx , cx . tcx . parent (def_id)) }
};
}
