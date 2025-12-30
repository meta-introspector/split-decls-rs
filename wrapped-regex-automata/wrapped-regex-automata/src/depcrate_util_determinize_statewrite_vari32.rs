// Generated macro for write_vari32 (function)
macro_rules! Depcrate_util_determinize_statewrite_vari32 {
() => {
// Module: crate::util::determinize::state
// Provides: {"write_vari32"}
// Dependencies: {}
# [doc = " Write a signed 32-bit integer using zig-zag encoding."] # [doc = ""] # [doc = " https://developers.google.com/protocol-buffers/docs/encoding#varints"] fn write_vari32 (data : & mut Vec < u8 > , n : i32) { let mut un = n . to_bits () << 1 ; if n < 0 { un = ! un ; } write_varu32 (data , un) }
};
}
