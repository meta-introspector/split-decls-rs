// Generated macro for write_varu32 (function)
macro_rules! Depcrate_dfawrite_varu32 {
() => {
// Module: crate::dfa
// Provides: {"write_varu32"}
// Dependencies: {}
# [doc = " https://developers.google.com/protocol-buffers/docs/encoding#varints"] fn write_varu32 (data : & mut Vec < u8 > , mut n : u32) { while n >= 0b1000_0000 { data . push ((n as u8) | 0b1000_0000) ; n >>= 7 ; } data . push (n as u8) ; }
};
}
