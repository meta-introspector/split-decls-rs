// Generated macro for impl_146 (impl)
macro_rules! Depcrate_escapeimpl_146 {
() => {
// Module: crate::escape
// Provides: {"impl_146"}
// Dependencies: {}
impl Unescape { # [doc = " Returns this value as `char`, panicking if it's a byte with a value > 0x7f."] pub (crate) fn unwrap_char (self) -> char { match self { Self :: Byte (b) => { assert ! (b <= 0x7F , "non ASCII byte") ; b . into () } Self :: Unicode (c) => c , } } # [doc = " Returns this value as `u8`, panicking if it was `Unicode`."] pub (crate) fn unwrap_byte (self) -> u8 { match self { Self :: Byte (b) => b , Self :: Unicode (_) => panic ! ("unexpected unicode escape value") , } } }
};
}
