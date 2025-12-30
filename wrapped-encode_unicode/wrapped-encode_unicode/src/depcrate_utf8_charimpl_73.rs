// Generated macro for impl_73 (impl)
macro_rules! Depcrate_utf8_charimpl_73 {
() => {
// Module: crate::utf8_char
// Provides: {"impl_73"}
// Dependencies: {}
impl str :: FromStr for Utf8Char { type Err = FromStrError ; # [doc = " Create an `Utf8Char` from a string slice."] # [doc = " The string must contain exactly one codepoint."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use encode_unicode::error::FromStrError::*;"] # [doc = " use encode_unicode::Utf8Char;"] # [doc = " use std::str::FromStr;"] # [doc = ""] # [doc = " assert_eq!(Utf8Char::from_str(\"a\"), Ok(Utf8Char::from('a')));"] # [doc = " assert_eq!(Utf8Char::from_str(\"🂠\"), Ok(Utf8Char::from('🂠')));"] # [doc = " assert_eq!(Utf8Char::from_str(\"\"), Err(Empty));"] # [doc = " assert_eq!(Utf8Char::from_str(\"ab\"), Err(MultipleCodepoints));"] # [doc = " assert_eq!(Utf8Char::from_str(\"e\u{301}\"), Err(MultipleCodepoints));// 'e'+u301 combining mark"] # [doc = " ```"] fn from_str (s : & str) -> Result < Self , FromStrError > { if s . is_empty () { Err (FromStrError :: Empty) } else if s . len () != 1 + s . as_bytes () [0] . extra_utf8_bytes_unchecked () { Err (FromStrError :: MultipleCodepoints) } else { let mut bytes = [0 ; 4] ; bytes [.. s . len ()] . copy_from_slice (s . as_bytes ()) ; Ok (Utf8Char { bytes }) } } }
};
}
