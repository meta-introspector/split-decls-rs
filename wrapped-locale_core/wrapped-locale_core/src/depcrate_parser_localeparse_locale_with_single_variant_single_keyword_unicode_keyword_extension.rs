// Generated macro for parse_locale_with_single_variant_single_keyword_unicode_keyword_extension (function)
macro_rules! Depcrate_parser_localeparse_locale_with_single_variant_single_keyword_unicode_keyword_extension {
() => {
// Module: crate::parser::locale
// Provides: {"parse_locale_with_single_variant_single_keyword_unicode_keyword_extension"}
// Dependencies: {}
# [expect (clippy :: type_complexity)] pub const fn parse_locale_with_single_variant_single_keyword_unicode_keyword_extension (t : & [u8] , mode : ParserMode ,) -> Result < (subtags :: Language , Option < subtags :: Script > , Option < subtags :: Region > , Option < subtags :: Variant > , Option < (extensions :: unicode :: Key , Option < Subtag >) > ,) , ParseError , > { let iter = SubtagIterator :: new (t) ; parse_locale_with_single_variant_single_keyword_unicode_extension_from_iter (iter , mode) }
};
}
