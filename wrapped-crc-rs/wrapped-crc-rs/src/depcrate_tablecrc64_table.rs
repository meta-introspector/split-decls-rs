// Generated macro for crc64_table (function)
macro_rules! Depcrate_tablecrc64_table {
() => {
// Module: crate::table
// Provides: {"crc64_table"}
// Dependencies: {}
pub (crate) const fn crc64_table < const L : usize > (width : u8 , poly : u64 , reflect : bool ,) -> [[u64 ; 256] ; L] { let poly = if reflect { let poly = poly . reverse_bits () ; poly >> (64u8 - width) } else { poly << (64u8 - width) } ; let mut table = [[0u64 ; 256] ; L] ; let mut i = 0 ; while i < 256 && L > 0 { table [0] [i] = crc64 (poly , reflect , i as u64) ; i += 1 ; } if L > 1 { let mut i = 0 ; while i < 256 { let mut e = 1 ; while e < L { let one_lower = table [e - 1] [i] ; if reflect { table [e] [i] = (one_lower >> 8) ^ table [0] [(one_lower & 0xFF) as usize] ; } else { table [e] [i] = (one_lower << 8) ^ table [0] [((one_lower >> 56) & 0xFF) as usize] ; } e += 1 ; } i += 1 ; } } table }
};
}
