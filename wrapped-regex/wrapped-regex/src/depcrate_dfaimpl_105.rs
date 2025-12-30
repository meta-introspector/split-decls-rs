// Generated macro for impl_105 (impl)
macro_rules! Depcrate_dfaimpl_105 {
() => {
// Module: crate::dfa
// Provides: {"impl_105"}
// Dependencies: {}
impl Byte { fn byte (b : u8) -> Self { Byte (b as u16) } fn eof () -> Self { Byte (256) } fn is_eof (& self) -> bool { self . 0 == 256 } fn is_ascii_word (& self) -> bool { let b = match self . as_byte () { None => return false , Some (b) => b , } ; match b { b'A' ..= b'Z' | b'a' ..= b'z' | b'0' ..= b'9' | b'_' => true , _ => false , } } fn as_byte (& self) -> Option < u8 > { if self . is_eof () { None } else { Some (self . 0 as u8) } } }
};
}
