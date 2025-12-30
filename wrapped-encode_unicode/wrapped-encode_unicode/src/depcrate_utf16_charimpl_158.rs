// Generated macro for impl_158 (impl)
macro_rules! Depcrate_utf16_charimpl_158 {
() => {
// Module: crate::utf16_char
// Provides: {"impl_158"}
// Dependencies: {}
impl FromStr for Utf16Char { type Err = FromStrError ; # [doc = " Create an `Utf16Char` from a string slice."] # [doc = " The string must contain exactly one codepoint."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use encode_unicode::error::FromStrError::*;"] # [doc = " use encode_unicode::Utf16Char;"] # [doc = " use std::str::FromStr;"] # [doc = ""] # [doc = " assert_eq!(Utf16Char::from_str(\"a\"), Ok(Utf16Char::from('a')));"] # [doc = " assert_eq!(Utf16Char::from_str(\"🂠\"), Ok(Utf16Char::from('🂠')));"] # [doc = " assert_eq!(Utf16Char::from_str(\"\"), Err(Empty));"] # [doc = " assert_eq!(Utf16Char::from_str(\"ab\"), Err(MultipleCodepoints));"] # [doc = " assert_eq!(Utf16Char::from_str(\"e\u{301}\"), Err(MultipleCodepoints));// 'e'+u301 combining mark"] # [doc = " ```"] fn from_str (s : & str) -> Result < Self , FromStrError > { match Utf16Char :: from_str_start (s) { Ok ((u16c , bytes)) if bytes == s . len () => Ok (u16c) , Ok ((_ , _)) => Err (FromStrError :: MultipleCodepoints) , Err (EmptyStrError) => Err (FromStrError :: Empty) , } } }
};
}
