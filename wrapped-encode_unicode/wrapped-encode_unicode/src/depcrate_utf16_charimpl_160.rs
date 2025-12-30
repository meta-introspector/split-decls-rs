// Generated macro for impl_160 (impl)
macro_rules! Depcrate_utf16_charimpl_160 {
() => {
// Module: crate::utf16_char
// Provides: {"impl_160"}
// Dependencies: {}
impl From < Utf8Char > for Utf16Char { fn from (utf8 : Utf8Char) -> Utf16Char { let (b , utf8_len) = utf8 . to_array () ; match utf8_len { 1 => Utf16Char { units : [b [0] as u16 , 0] } , 4 => { let mut first = 0xd800 - (0x01_00_00u32 >> 10) as u16 ; first += (b [0] as u16 & 0x07) << 8 ; first += (b [1] as u16 & 0x3f) << 2 ; first += (b [2] as u16 & 0x30) >> 4 ; let mut second = 0xdc00 ; second |= (b [2] as u16 & 0x0f) << 6 ; second |= b [3] as u16 & 0x3f ; Utf16Char { units : [first , second] } } , _ => { let mut unit = ((b [0] as u16 & 0x1f) << 6) | (b [1] as u16 & 0x3f) ; if utf8_len == 3 { unit = (unit << 6) | (b [2] as u16 & 0x3f) ; } Utf16Char { units : [unit , 0] } } , } } }
};
}
