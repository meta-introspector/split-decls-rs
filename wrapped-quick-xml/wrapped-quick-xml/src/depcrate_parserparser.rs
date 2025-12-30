// Generated macro for Parser (trait)
macro_rules! Depcrate_parserParser {
() => {
// Module: crate::parser
// Provides: {"Parser"}
// Dependencies: {}
# [doc = " Used to decouple reading of data from data source and parsing XML structure from it."] # [doc = " This is a state preserved between getting chunks of bytes from the reader."] # [doc = ""] # [doc = " This trait is implemented for every parser that processes piece of XML grammar."] pub trait Parser { # [doc = " Process new data and try to determine end of the parsed thing."] # [doc = ""] # [doc = " Returns position of the end of thing in `bytes` in case of successful search"] # [doc = " and `None` otherwise."] # [doc = ""] # [doc = " # Parameters"] # [doc = " - `bytes`: a slice to find the end of a thing."] # [doc = "   Should contain text in ASCII-compatible encoding"] fn feed (& mut self , bytes : & [u8]) -> Option < usize > ; # [doc = " Returns parse error produced by this parser in case of reaching end of"] # [doc = " input without finding the end of a parsed thing."] fn eof_error () -> SyntaxError ; }
};
}
