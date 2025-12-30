// Generated macro for ParseError (struct)
macro_rules! Depcrate_parseParseError {
() => {
// Module: crate::parse
// Provides: {"ParseError"}
// Dependencies: {}
# [doc = " The error that was encountered while parsing an encoding string."] # [derive (Debug , PartialEq , Eq , Hash)] pub struct ParseError { kind : ErrorKind , data : String , split_point : usize , }
};
}
