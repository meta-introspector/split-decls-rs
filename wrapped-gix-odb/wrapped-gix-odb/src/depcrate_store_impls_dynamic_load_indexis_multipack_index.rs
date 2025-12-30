// Generated macro for is_multipack_index (function)
macro_rules! Depcrate_store_impls_dynamic_load_indexis_multipack_index {
() => {
// Module: crate::store_impls::dynamic::load_index
// Provides: {"is_multipack_index"}
// Dependencies: {}
fn is_multipack_index (path : & Path) -> bool { path . file_name () == Some (OsStr :: new ("multi-pack-index")) }
};
}
