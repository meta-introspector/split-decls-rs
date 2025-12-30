// Generated macro for write_vari32 (function)
macro_rules! Depcrate_dfawrite_vari32 {
() => {
// Module: crate::dfa
// Provides: {"write_vari32"}
// Dependencies: {}
# [doc = " https://developers.google.com/protocol-buffers/docs/encoding#varints"] fn write_vari32 (data : & mut Vec < u8 > , n : i32) { let mut un = (n as u32) << 1 ; if n < 0 { un = ! un ; } write_varu32 (data , un) }
};
}
