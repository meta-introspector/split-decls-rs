// Generated macro for combine (function)
macro_rules! Depcrate_combinecombine {
() => {
// Module: crate::combine
// Provides: {"combine"}
// Dependencies: {}
pub (crate) fn combine (crc1 : u32 , crc2 : u32 , len2 : u64) -> u32 { if len2 == 0 { return crc1 ; } let mut p = crc1 ; let n = 64 - len2 . leading_zeros () ; for i in 0 .. n { if (len2 >> i & 1) != 0 { p = multiply (X2N_TABLE [(i & 0x1F) as usize] , p) ; } } p ^ crc2 }
};
}
