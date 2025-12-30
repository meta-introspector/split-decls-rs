// Generated macro for varint_u16 (function)
macro_rules! Depcrate_varintvarint_u16 {
() => {
// Module: crate::varint
// Provides: {"varint_u16"}
// Dependencies: {}
# [inline] pub fn varint_u16 (n : u16 , out : & mut [u8 ; varint_max :: < u16 > ()]) -> & mut [u8] { let mut value = n ; for i in 0 .. varint_max :: < u16 > () { out [i] = value . to_le_bytes () [0] ; if value < 128 { return & mut out [..= i] ; } out [i] |= 0x80 ; value >>= 7 ; } debug_assert_eq ! (value , 0) ; & mut out [..] }
};
}
