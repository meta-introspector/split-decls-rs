// Generated macro for crc8_table (function)
macro_rules! Depcrate_tablecrc8_table {
() => {
// Module: crate::table
// Provides: {"crc8_table"}
// Dependencies: {}
pub (crate) const fn crc8_table < const L : usize > (width : u8 , poly : u8 , reflect : bool ,) -> [[u8 ; 256] ; L] { let poly = if reflect { let poly = poly . reverse_bits () ; poly >> (8u8 - width) } else { poly << (8u8 - width) } ; let mut table = [[0u8 ; 256] ; L] ; let mut i = 0 ; while i < 256 && L > 0 { table [0] [i] = crc8 (poly , reflect , i as u8) ; i += 1 ; } if L > 1 { let mut i = 0 ; while i < 256 { let mut e = 1 ; while e < L { let one_lower = table [e - 1] [i] ; table [e] [i] = table [0] [one_lower as usize] ; e += 1 ; } i += 1 ; } } table }
};
}
