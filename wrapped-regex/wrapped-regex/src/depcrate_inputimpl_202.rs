// Generated macro for impl_202 (impl)
macro_rules! Depcrate_inputimpl_202 {
() => {
// Module: crate::input
// Provides: {"impl_202"}
// Dependencies: {}
impl Char { # [doc = " Returns true iff the character is absent."] # [inline] pub fn is_none (self) -> bool { self . 0 == u32 :: MAX } # [doc = " Returns the length of the character's UTF-8 encoding."] # [doc = ""] # [doc = " If the character is absent, then `0` is returned."] # [inline] pub fn len_utf8 (self) -> usize { char :: from_u32 (self . 0) . map_or (0 , | c | c . len_utf8 ()) } # [doc = " Returns true iff the character is a word character."] # [doc = ""] # [doc = " If the character is absent, then false is returned."] pub fn is_word_char (self) -> bool { char :: from_u32 (self . 0) . map_or (false , syntax :: is_word_char) } # [doc = " Returns true iff the byte is a word byte."] # [doc = ""] # [doc = " If the byte is absent, then false is returned."] pub fn is_word_byte (self) -> bool { match char :: from_u32 (self . 0) { None => false , Some (c) if c <= '\u{7F}' => syntax :: is_word_byte (c as u8) , Some (_) => false , } } # [doc = " Converts the character to a real primitive `char`."] # [doc = ""] # [doc = " If the character is absent, then `None` is returned."] pub fn as_char (self) -> Option < char > { char :: from_u32 (self . 0) } }
};
}
