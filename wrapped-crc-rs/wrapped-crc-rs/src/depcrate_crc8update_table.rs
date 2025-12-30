// Generated macro for update_table (function)
macro_rules! Depcrate_crc8update_table {
() => {
// Module: crate::crc8
// Provides: {"update_table"}
// Dependencies: {}
const fn update_table < const L : usize > (mut crc : u8 , algorithm : & Algorithm < u8 > , table : & [[u8 ; 256] ; L] , bytes : & [u8] ,) -> u8 { let len = bytes . len () ; let mut i = 0 ; if L == 16 { while i + 16 <= len { crc = table [0] [bytes [i + 15] as usize] ^ table [1] [bytes [i + 14] as usize] ^ table [2] [bytes [i + 13] as usize] ^ table [3] [bytes [i + 12] as usize] ^ table [4] [bytes [i + 11] as usize] ^ table [5] [bytes [i + 10] as usize] ^ table [6] [bytes [i + 9] as usize] ^ table [7] [bytes [i + 8] as usize] ^ table [8] [bytes [i + 7] as usize] ^ table [9] [bytes [i + 6] as usize] ^ table [10] [bytes [i + 5] as usize] ^ table [11] [bytes [i + 4] as usize] ^ table [12] [bytes [i + 3] as usize] ^ table [13] [bytes [i + 2] as usize] ^ table [14] [bytes [i + 1] as usize] ^ table [15] [(bytes [i] ^ crc) as usize] ; i += 16 ; } } if L > 0 { while i < len { crc = table [0] [(crc ^ bytes [i]) as usize] ; i += 1 ; } } else { let poly = if algorithm . refin { let poly = algorithm . poly . reverse_bits () ; poly >> (8u8 - algorithm . width) } else { algorithm . poly << (8u8 - algorithm . width) } ; while i < len { crc = crc8 (poly , algorithm . refin , crc ^ bytes [i]) ; i += 1 ; } } crc }
};
}
