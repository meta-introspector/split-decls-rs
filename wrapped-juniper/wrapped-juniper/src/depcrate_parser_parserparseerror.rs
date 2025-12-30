// Generated macro for ParseError (enum)
macro_rules! Depcrate_parser_parserParseError {
() => {
// Module: crate::parser::parser
// Provides: {"ParseError"}
// Dependencies: {}
# [doc = " Error while parsing a GraphQL query"] # [derive (Clone , Debug , Display , Eq , Error , From , PartialEq)] pub enum ParseError { # [doc = " An unexpected token occurred in the source"] # [display ("Unexpected \"{_0}\"")] UnexpectedToken (# [error (not (source))] CompactString) , # [doc = " The input source abruptly ended"] # [display ("Unexpected end of input")] UnexpectedEndOfFile , # [doc = " An error during tokenization occurred"] # [from] LexerError (LexerError) , # [doc = " A scalar of unexpected type occurred in the source"] ExpectedScalarError (# [error (not (source))] & 'static str) , }
};
}
