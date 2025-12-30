// Generated macro for quiche_put_varint (function)
macro_rules! Depcrate_ffiquiche_put_varint {
() => {
// Module: crate::ffi
// Provides: {"quiche_put_varint"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_put_varint (buf : * mut u8 , buf_len : size_t , val : u64 ,) -> c_int { let buf = unsafe { slice :: from_raw_parts_mut (buf , buf_len) } ; let mut b = octets :: OctetsMut :: with_slice (buf) ; match b . put_varint (val) { Ok (_) => 0 , Err (e) => { let err : Error = e . into () ; err . to_c () as c_int } , } }
};
}
