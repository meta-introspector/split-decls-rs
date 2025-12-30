// Generated macro for crc32_table (function)
macro_rules! Depcrate_tablecrc32_table {
() => {
// Module: crate::table
// Provides: {"crc32_table"}
// Dependencies: {}
pub (crate) const fn crc32_table < const L : usize > (width : u8 , poly : u32 , reflect : bool ,) -> [[u32 ; 256] ; L] { let poly = if reflect { let poly = poly . reverse_bits () ; poly >> (32u8 - width) } else { poly << (32u8 - width) } ; let mut table = [[0u32 ; 256] ; L] ; let mut i = 0 ; while i < 256 && L > 0 { table [0] [i] = crc32 (poly , reflect , i as u32) ; i += 1 ; } if L > 1 { let mut i = 0 ; while i < 256 { let mut e = 1 ; while e < L { let one_lower = table [e - 1] [i] ; if reflect { table [e] [i] = (one_lower >> 8) ^ table [0] [(one_lower & 0xFF) as usize] ; } else { table [e] [i] = (one_lower << 8) ^ table [0] [((one_lower >> 24) & 0xFF) as usize] ; } e += 1 ; } i += 1 ; } } table }
};
}
