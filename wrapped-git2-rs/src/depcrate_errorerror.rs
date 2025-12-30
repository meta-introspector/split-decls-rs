// Generated macro for Error (struct)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " A structure to represent errors coming out of libgit2."] # [derive (Debug , PartialEq)] pub struct Error { code : c_int , klass : c_int , message : Box < str > , }
};
}
