// Generated macro for ErrorKind (enum)
macro_rules! Depcrate_errorErrorKind {
() => {
// Module: crate::error
// Provides: {"ErrorKind"}
// Dependencies: {}
# [derive (Debug)] pub enum ErrorKind { None , Str (str :: Utf8Error) , Regex (regex :: Error) , Nul (ffi :: NulError) , }
};
}
