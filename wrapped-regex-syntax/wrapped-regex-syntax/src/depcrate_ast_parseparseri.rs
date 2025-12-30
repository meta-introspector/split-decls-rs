// Generated macro for ParserI (struct)
macro_rules! Depcrate_ast_parseParserI {
() => {
// Module: crate::ast::parse
// Provides: {"ParserI"}
// Dependencies: {}
# [doc = " ParserI is the internal parser implementation."] # [doc = ""] # [doc = " We use this separate type so that we can carry the provided pattern string"] # [doc = " along with us. In particular, a `Parser` internal state is not tied to any"] # [doc = " one pattern, but `ParserI` is."] # [doc = ""] # [doc = " This type also lets us use `ParserI<&Parser>` in production code while"] # [doc = " retaining the convenience of `ParserI<Parser>` for tests, which sometimes"] # [doc = " work against the internal interface of the parser."] # [derive (Clone , Debug)] struct ParserI < 's , P > { # [doc = " The parser state/configuration."] parser : P , # [doc = " The full regular expression provided by the user."] pattern : & 's str , }
};
}
