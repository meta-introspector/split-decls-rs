// Generated macro for parse_language_identifier (function)
macro_rules! Depcrate_parser_langidparse_language_identifier {
() => {
// Module: crate::parser::langid
// Provides: {"parse_language_identifier"}
// Dependencies: {}
# [cfg (feature = "alloc")] pub fn parse_language_identifier (t : & [u8] , mode : ParserMode ,) -> Result < LanguageIdentifier , ParseError > { let mut iter = SubtagIterator :: new (t) ; parse_language_identifier_from_iter (& mut iter , mode) }
};
}
