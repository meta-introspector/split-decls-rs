// Generated macro for varint_usize (function)
macro_rules! Depcrate_varintvarint_usize {
() => {
// Module: crate::varint
// Provides: {"varint_usize"}
// Dependencies: {}
# [inline] pub fn varint_usize (n : usize , out : & mut [u8 ; varint_max :: < usize > ()]) -> & mut [u8] { let mut value = n ; for i in 0 .. varint_max :: < usize > () { out [i] = value . to_le_bytes () [0] ; if value < 128 { return & mut out [..= i] ; } out [i] |= 0x80 ; value >>= 7 ; } debug_assert_eq ! (value , 0) ; & mut out [..] }
};
}
