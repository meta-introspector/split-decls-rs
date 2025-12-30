// Generated macro for parse_crate_attrs (function)
macro_rules! Depcrateparse_crate_attrs {
() => {
// Module: crate
// Provides: {"parse_crate_attrs"}
// Dependencies: {}
fn parse_crate_attrs < 'a > (sess : & 'a Session) -> PResult < 'a , ast :: AttrVec > { let mut parser = unwrap_or_emit_fatal (match & sess . io . input { Input :: File (file) => { new_parser_from_file (& sess . psess , file , StripTokens :: ShebangAndFrontmatter , None) } Input :: Str { name , input } => new_parser_from_source_str (& sess . psess , name . clone () , input . clone () , StripTokens :: ShebangAndFrontmatter ,) , }) ; parser . parse_inner_attributes () }
};
}
