// Generated macro for parse_language_identifier_with_single_variant (function)
macro_rules! Depcrate_parser_langidparse_language_identifier_with_single_variant {
() => {
// Module: crate::parser::langid
// Provides: {"parse_language_identifier_with_single_variant"}
// Dependencies: {}
# [expect (clippy :: type_complexity)] pub const fn parse_language_identifier_with_single_variant (t : & [u8] , mode : ParserMode ,) -> Result < (subtags :: Language , Option < subtags :: Script > , Option < subtags :: Region > , Option < subtags :: Variant > ,) , ParseError , > { let iter = SubtagIterator :: new (t) ; match parse_locale_with_single_variant_single_keyword_unicode_extension_from_iter (iter , mode) { Ok ((l , s , r , v , _)) => Ok ((l , s , r , v)) , Err (e) => Err (e) , } }
};
}
