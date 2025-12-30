// Generated macro for ParseError (enum)
macro_rules! DepcrateParseError {
() => {
// Module: crate
// Provides: {"ParseError"}
// Dependencies: {}
# [derive (Debug)] pub enum ParseError { MissingSlash , MissingEqual , MissingQuote , InvalidToken { pos : usize , byte : Byte , } , InvalidRange , TooLong , }
};
}
