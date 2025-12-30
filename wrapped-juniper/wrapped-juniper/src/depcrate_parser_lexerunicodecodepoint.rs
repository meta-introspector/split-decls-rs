// Generated macro for UnicodeCodePoint (struct)
macro_rules! Depcrate_parser_lexerUnicodeCodePoint {
() => {
// Module: crate::parser::lexer
// Provides: {"UnicodeCodePoint"}
// Dependencies: {}
# [doc = " Representation of a [Unicode code point]."] # [doc = ""] # [doc = " This is different from a [Unicode scalar value] (aka \"character\") represented by a [`char`],"] # [doc = " because can denote a [surrogate code point]."] # [doc = ""] # [doc = " [surrogate code point]: https://unicode.org/glossary#surrogate_code_point"] # [doc = " [Unicode code point]: https://unicode.org/glossary#code_point"] # [doc = " [Unicode scalar value]: https://unicode.org/glossary#unicode_scalar_value"] # [derive (Clone , Copy , Debug)] pub (crate) struct UnicodeCodePoint { # [doc = " Code representing this [`UnicodeCodePoint`]."] pub (crate) code : u32 , # [doc = " Indicator whether this [`UnicodeCodePoint`] should be [`Display`]ed in variable-width form."] pub (crate) is_variable_width : bool , }
};
}
