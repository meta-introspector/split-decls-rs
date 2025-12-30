// Generated macro for parse_locale (function)
macro_rules! Depcrate_parser_localeparse_locale {
() => {
// Module: crate::parser::locale
// Provides: {"parse_locale"}
// Dependencies: {}
# [cfg (feature = "alloc")] pub fn parse_locale (t : & [u8]) -> Result < Locale , ParseError > { let mut iter = SubtagIterator :: new (t) ; let id = super :: parse_language_identifier_from_iter (& mut iter , ParserMode :: Locale) ? ; let extensions = if iter . peek () . is_some () { extensions :: Extensions :: try_from_iter (& mut iter) ? } else { extensions :: Extensions :: default () } ; Ok (Locale { id , extensions }) }
};
}
