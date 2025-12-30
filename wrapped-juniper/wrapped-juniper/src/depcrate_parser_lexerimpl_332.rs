// Generated macro for impl_332 (impl)
macro_rules! Depcrate_parser_lexerimpl_332 {
() => {
// Module: crate::parser::lexer
// Provides: {"impl_332"}
// Dependencies: {}
impl UnicodeCodePoint { # [doc = " Indicates whether this [`UnicodeCodePoint`] is a high (leading) [surrogate]."] # [doc = ""] # [doc = " [surrogate]: https://unicode.org/glossary#surrogate_code_point"] pub (crate) fn is_high_surrogate (self) -> bool { (0xD800 ..= 0xDBFF) . contains (& self . code) } # [doc = " Indicates whether this [`UnicodeCodePoint`] is a low (trailing) [surrogate]."] # [doc = ""] # [doc = " [surrogate]: https://unicode.org/glossary#surrogate_code_point"] pub (crate) fn is_low_surrogate (self) -> bool { (0xDC00 ..= 0xDFFF) . contains (& self . code) } # [doc = " Joins a [`UnicodeCodePoint`] from the provided [surrogate pair][0]."] # [doc = ""] # [doc = " [0]: https://unicodebook.readthedocs.io/unicode_encodings.html#utf-16-surrogate-pairs"] pub (crate) fn from_surrogate_pair (high : Self , low : Self) -> Self { debug_assert ! (high . is_high_surrogate () , "`{high}` is not a high surrogate") ; debug_assert ! (low . is_low_surrogate () , "`{high}` is not a low surrogate") ; Self { code : 0x10000 + ((high . code & 0x03FF) << 10) + (low . code & 0x03FF) , is_variable_width : true , } } # [doc = " Tries to convert this [`UnicodeCodePoint`] into a [`char`]."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " If this [`UnicodeCodePoint`] doesn't represent a [Unicode scalar value]."] # [doc = ""] # [doc = " [Unicode scalar value]: https://unicode.org/glossary#unicode_scalar_value"] pub (crate) fn try_into_char (self) -> Result < char , LexerError > { char :: from_u32 (self . code) . ok_or_else (| | LexerError :: UnknownEscapeSequence (self . to_string ())) } }
};
}
