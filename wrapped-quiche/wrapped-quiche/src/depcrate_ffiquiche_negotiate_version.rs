// Generated macro for quiche_negotiate_version (function)
macro_rules! Depcrate_ffiquiche_negotiate_version {
() => {
// Module: crate::ffi
// Provides: {"quiche_negotiate_version"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_negotiate_version (scid : * const u8 , scid_len : size_t , dcid : * const u8 , dcid_len : size_t , out : * mut u8 , out_len : size_t ,) -> ssize_t { let scid = unsafe { slice :: from_raw_parts (scid , scid_len) } ; let scid = ConnectionId :: from_ref (scid) ; let dcid = unsafe { slice :: from_raw_parts (dcid , dcid_len) } ; let dcid = ConnectionId :: from_ref (dcid) ; let out = unsafe { slice :: from_raw_parts_mut (out , out_len) } ; match negotiate_version (& scid , & dcid , out) { Ok (v) => v as ssize_t , Err (e) => e . to_c () , } }
};
}
