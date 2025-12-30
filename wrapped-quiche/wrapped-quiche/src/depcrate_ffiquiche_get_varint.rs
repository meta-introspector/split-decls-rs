// Generated macro for quiche_get_varint (function)
macro_rules! Depcrate_ffiquiche_get_varint {
() => {
// Module: crate::ffi
// Provides: {"quiche_get_varint"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_get_varint (buf : * const u8 , buf_len : size_t , val : * mut u64 ,) -> ssize_t { let buf = unsafe { slice :: from_raw_parts (buf , buf_len) } ; let mut b = octets :: Octets :: with_slice (buf) ; match b . get_varint () { Ok (v) => unsafe { * val = v } , Err (e) => { let err : Error = e . into () ; return err . to_c () ; } , } ; b . off () as ssize_t }
};
}
