// Generated macro for impl_74 (impl)
macro_rules! Depcrate_utf8_charimpl_74 {
() => {
// Module: crate::utf8_char
// Provides: {"impl_74"}
// Dependencies: {}
impl From < Utf16Char > for Utf8Char { fn from (utf16 : Utf16Char) -> Utf8Char { match utf16 . to_tuple () { (ascii @ 0 ..= 0x00_7f , _) => { Utf8Char { bytes : [ascii as u8 , 0 , 0 , 0] } } , (unit @ 0 ..= 0x07_ff , _) => { let byte2 = 0x80 | (unit & 0x00_3f) as u8 ; let byte1 = 0xc0 | ((unit & 0x07_c0) >> 6) as u8 ; Utf8Char { bytes : [byte1 , byte2 , 0 , 0] } } , (unit , None) => { let byte3 = 0x80 | (unit & 0x00_3f) as u8 ; let byte2 = 0x80 | ((unit & 0x0f_c0) >> 6) as u8 ; let byte1 = 0xe0 | ((unit & 0xf0_00) >> 12) as u8 ; Utf8Char { bytes : [byte1 , byte2 , byte3 , 0] } } , (first , Some (second)) => { let first = first + (0x01_00_00u32 >> 10) as u16 ; let byte4 = 0x80 | (second & 0x00_3f) as u8 ; let byte3 = 0x80 | ((second & 0x03_c0) >> 6) as u8 | ((first & 0x00_03) << 4) as u8 ; let byte2 = 0x80 | ((first & 0x00_fc) >> 2) as u8 ; let byte1 = 0xf0 | ((first & 0x07_00) >> 8) as u8 ; Utf8Char { bytes : [byte1 , byte2 , byte3 , byte4] } } } } }
};
}
