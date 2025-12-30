// Generated macro for quiche_retry (function)
macro_rules! Depcrate_ffiquiche_retry {
() => {
// Module: crate::ffi
// Provides: {"quiche_retry"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_retry (scid : * const u8 , scid_len : size_t , dcid : * const u8 , dcid_len : size_t , new_scid : * const u8 , new_scid_len : size_t , token : * const u8 , token_len : size_t , version : u32 , out : * mut u8 , out_len : size_t ,) -> ssize_t { let scid = unsafe { slice :: from_raw_parts (scid , scid_len) } ; let scid = ConnectionId :: from_ref (scid) ; let dcid = unsafe { slice :: from_raw_parts (dcid , dcid_len) } ; let dcid = ConnectionId :: from_ref (dcid) ; let new_scid = unsafe { slice :: from_raw_parts (new_scid , new_scid_len) } ; let new_scid = ConnectionId :: from_ref (new_scid) ; let token = unsafe { slice :: from_raw_parts (token , token_len) } ; let out = unsafe { slice :: from_raw_parts_mut (out , out_len) } ; match retry (& scid , & dcid , & new_scid , token , version , out) { Ok (v) => v as ssize_t , Err (e) => e . to_c () , } }
};
}
