// Generated macro for impl_307 (impl)
macro_rules! Depcrate_parserimpl_307 {
() => {
// Module: crate::parser
// Provides: {"impl_307"}
// Dependencies: {}
impl Parser { # [doc = " Create a new parser with a default configuration."] # [doc = ""] # [doc = " The parser can be run with `parse` method. The parse method returns"] # [doc = " a high level intermediate representation of the given regular"] # [doc = " expression."] # [doc = ""] # [doc = " To set configuration options on the parser, use [`ParserBuilder`]."] pub fn new () -> Parser { ParserBuilder :: new () . build () } # [doc = " Parse the regular expression into a high level intermediate"] # [doc = " representation."] pub fn parse (& mut self , pattern : & str) -> Result < hir :: Hir , Error > { let ast = self . ast . parse (pattern) ? ; let hir = self . hir . translate (pattern , & ast) ? ; Ok (hir) } }
};
}
