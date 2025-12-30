// Generated macro for crc16_table (function)
macro_rules! Depcrate_tablecrc16_table {
() => {
// Module: crate::table
// Provides: {"crc16_table"}
// Dependencies: {}
pub (crate) const fn crc16_table < const L : usize > (width : u8 , poly : u16 , reflect : bool ,) -> [[u16 ; 256] ; L] { let poly = if reflect { let poly = poly . reverse_bits () ; poly >> (16u8 - width) } else { poly << (16u8 - width) } ; let mut table = [[0u16 ; 256] ; L] ; let mut i = 0 ; while i < 256 && L > 0 { table [0] [i] = crc16 (poly , reflect , i as u16) ; i += 1 ; } if L > 1 { let mut i = 0 ; while i < 256 { let mut e = 1 ; while e < L { let one_lower = table [e - 1] [i] ; if reflect { table [e] [i] = (one_lower >> 8) ^ table [0] [(one_lower & 0xFF) as usize] ; } else { table [e] [i] = (one_lower << 8) ^ table [0] [((one_lower >> 8) & 0xFF) as usize] ; } e += 1 ; } i += 1 ; } } table }
};
}
