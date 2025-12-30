// Generated macro for quiche_conn_source_ids (function)
macro_rules! Depcrate_ffiquiche_conn_source_ids {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_source_ids"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_source_ids (conn : & Connection ,) -> * mut ConnectionIdIter < '_ > { let vec = conn . source_ids () . cloned () . collect () ; Box :: into_raw (Box :: new (ConnectionIdIter { cids : vec , index : 0 , })) }
};
}
