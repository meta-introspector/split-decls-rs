// Generated macro for ParseError (struct)
macro_rules! DepcrateParseError {
() => {
// Module: crate
// Provides: {"ParseError"}
// Dependencies: {}
# [derive (Debug)] # [doc = " Parse error"] pub struct ParseError { pub line : usize , pub col : usize , pub msg : Cow < 'static , str > , }
};
}
