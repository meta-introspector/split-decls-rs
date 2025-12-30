// Generated macro for quiche_conn_retire_dcid (function)
macro_rules! Depcrate_ffiquiche_conn_retire_dcid {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_retire_dcid"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_retire_dcid (conn : & mut Connection , dcid_seq : u64 ,) -> c_int { match conn . retire_dcid (dcid_seq) { Ok (_) => 0 , Err (e) => e . to_c () as c_int , } }
};
}
