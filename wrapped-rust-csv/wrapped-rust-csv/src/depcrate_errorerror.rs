// Generated macro for Error (struct)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " An error that can occur when processing CSV data."] # [doc = ""] # [doc = " This error can happen when writing or reading CSV data."] # [doc = ""] # [doc = " There are some important scenarios where an error is impossible to occur."] # [doc = " For example, if a CSV reader is used on an in-memory buffer with the"] # [doc = " `flexible` option enabled and one is reading records as raw byte strings,"] # [doc = " then no error can occur."] # [derive (Debug)] pub struct Error (Box < ErrorKind >) ;
};
}
