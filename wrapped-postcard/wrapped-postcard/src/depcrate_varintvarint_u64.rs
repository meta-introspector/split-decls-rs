// Generated macro for varint_u64 (function)
macro_rules! Depcrate_varintvarint_u64 {
() => {
// Module: crate::varint
// Provides: {"varint_u64"}
// Dependencies: {}
# [inline] pub fn varint_u64 (n : u64 , out : & mut [u8 ; varint_max :: < u64 > ()]) -> & mut [u8] { if n < 128 { out [0] = n as u8 ; return & mut out [.. 1] ; } let mut value = n ; let mut i = 0 ; while value >= 128 { out [i] = value . to_le_bytes () [0] | 0b10000000 ; value >>= 7 ; i += 1 ; } out [i] = value as u8 ; & mut out [..= i] }
};
}
