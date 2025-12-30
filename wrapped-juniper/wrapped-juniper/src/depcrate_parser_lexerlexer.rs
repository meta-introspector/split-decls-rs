// Generated macro for Lexer (struct)
macro_rules! Depcrate_parser_lexerLexer {
() => {
// Module: crate::parser::lexer
// Provides: {"Lexer"}
// Dependencies: {}
# [doc (hidden)] # [derive (Debug)] pub struct Lexer < 'a > { iterator : itertools :: PeekNth < CharIndices < 'a > > , source : & 'a str , length : usize , position : SourcePosition , has_reached_eof : bool , }
};
}
