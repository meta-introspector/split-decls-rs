// Generated macro for Error (struct)
macro_rules! Depcrate_parserError {
() => {
// Module: crate::parser
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Describes a parse error and where in the input it occurs."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct Error < 'i > { input : & 'i str , pos : usize , error : & 'static str , }
};
}
