// Generated macro for Error (struct)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " A generic \"error\" for HTTP connections"] # [doc = ""] # [doc = " This error type is less specific than the error returned from other"] # [doc = " functions in this crate, but all other errors can be converted to this"] # [doc = " error. Consumers of this crate can typically consume and work with this form"] # [doc = " of error for conversions with the `?` operator."] pub struct Error { inner : ErrorKind , }
};
}
