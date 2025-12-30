// Generated macro for quiche_conn_new_scid (function)
macro_rules! Depcrate_ffiquiche_conn_new_scid {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_new_scid"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_new_scid (conn : & mut Connection , scid : * const u8 , scid_len : size_t , reset_token : * const u8 , retire_if_needed : bool , scid_seq : * mut u64 ,) -> c_int { let scid = unsafe { slice :: from_raw_parts (scid , scid_len) } ; let scid = ConnectionId :: from_ref (scid) ; let reset_token = unsafe { slice :: from_raw_parts (reset_token , 16) } ; let reset_token = match reset_token . try_into () { Ok (rt) => rt , Err (_) => unreachable ! () , } ; let reset_token = u128 :: from_be_bytes (reset_token) ; match conn . new_scid (& scid , reset_token , retire_if_needed) { Ok (c) => { unsafe { * scid_seq = c } 0 } , Err (e) => e . to_c () as c_int , } }
};
}
