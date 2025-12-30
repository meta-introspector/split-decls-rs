// Generated macro for impl_44 (impl)
macro_rules! Depcrate_traitsimpl_44 {
() => {
// Module: crate::traits
// Provides: {"impl_44"}
// Dependencies: {}
impl U8UtfExt for u8 { # [inline] fn extra_utf8_bytes (self) -> Result < usize , Utf8Error > { match self { 0x00 ..= 0x7f => Ok (0) , 0xc2 ..= 0xdf => Ok (1) , 0xe0 ..= 0xef => Ok (2) , 0xf0 ..= 0xf4 => Ok (3) , 0xc0 ..= 0xc1 | 0xf5 ..= 0xff => Err (Utf8Error { kind : NonUtf8Byte }) , 0x80 ..= 0xbf => Err (Utf8Error { kind : UnexpectedContinuationByte }) , } } # [inline] fn extra_utf8_bytes_unchecked (self) -> usize { if self < 128 { 0 } else { ((self as u32) << 25) . not () . leading_zeros () as usize } } }
};
}
