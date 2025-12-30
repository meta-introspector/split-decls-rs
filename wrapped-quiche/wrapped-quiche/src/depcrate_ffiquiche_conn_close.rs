// Generated macro for quiche_conn_close (function)
macro_rules! Depcrate_ffiquiche_conn_close {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_close"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_close (conn : & mut Connection , app : bool , err : u64 , reason : * const u8 , reason_len : size_t ,) -> c_int { let reason = if reason . is_null () { assert_eq ! (reason_len , 0) ; & [] } else { unsafe { slice :: from_raw_parts (reason , reason_len) } } ; match conn . close (app , err , reason) { Ok (_) => 0 , Err (e) => e . to_c () as c_int , } }
};
}
