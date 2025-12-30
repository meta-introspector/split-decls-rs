// Generated macro for read_varu32 (function)
macro_rules! Depcrate_dfaread_varu32 {
() => {
// Module: crate::dfa
// Provides: {"read_varu32"}
// Dependencies: {}
# [doc = " https://developers.google.com/protocol-buffers/docs/encoding#varints"] fn read_varu32 (data : & [u8]) -> (u32 , usize) { let mut n : u32 = 0 ; let mut shift : u32 = 0 ; for (i , & b) in data . iter () . enumerate () { if b < 0b1000_0000 { return (n | ((b as u32) << shift) , i + 1) ; } n |= ((b as u32) & 0b0111_1111) << shift ; shift += 7 ; } (0 , 0) }
};
}
