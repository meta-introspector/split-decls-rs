// Generated macro for impl_340 (impl)
macro_rules! Depcrate_parser_parserimpl_340 {
() => {
// Module: crate::parser::parser
// Provides: {"impl_340"}
// Dependencies: {}
impl ParseError { # [doc = " Creates a [`ParseError::UnexpectedToken`] out of the provided [`Token`]."] # [must_use] pub fn unexpected_token (token : Token < '_ >) -> Self { Self :: UnexpectedToken (format_compact ! ("{token}")) } }
};
}
