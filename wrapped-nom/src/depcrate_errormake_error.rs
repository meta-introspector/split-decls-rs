// Generated macro for make_error (function)
macro_rules! Depcrate_errormake_error {
() => {
// Module: crate::error
// Provides: {"make_error"}
// Dependencies: {}
# [doc = " Creates an error from the input position and an [ErrorKind]"] pub fn make_error < I , E : ParseError < I > > (input : I , kind : ErrorKind) -> E { E :: from_error_kind (input , kind) }
};
}
