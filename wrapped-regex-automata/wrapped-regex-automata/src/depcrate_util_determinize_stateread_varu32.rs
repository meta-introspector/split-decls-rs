// Generated macro for read_varu32 (function)
macro_rules! Depcrate_util_determinize_stateread_varu32 {
() => {
// Module: crate::util::determinize::state
// Provides: {"read_varu32"}
// Dependencies: {}
# [doc = " Read an unsigned 32-bit varint. Also, return the number of bytes read."] # [doc = ""] # [doc = " https://developers.google.com/protocol-buffers/docs/encoding#varints"] fn read_varu32 (data : & [u8]) -> (u32 , usize) { let mut n : u32 = 0 ; let mut shift : u32 = 0 ; for (i , & b) in data . iter () . enumerate () { if b < 0b1000_0000 { return (n | (u32 :: from (b) << shift) , i + 1) ; } n |= (u32 :: from (b) & 0b0111_1111) << shift ; shift += 7 ; } (0 , 0) }
};
}
