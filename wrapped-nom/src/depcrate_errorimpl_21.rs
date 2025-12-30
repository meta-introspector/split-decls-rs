// Generated macro for impl_21 (impl)
macro_rules! Depcrate_errorimpl_21 {
() => {
// Module: crate::error
// Provides: {"impl_21"}
// Dependencies: {}
impl < I , E > FromExternalError < I , E > for Error < I > { # [doc = " Create a new error from an input position and an external error"] fn from_external_error (input : I , kind : ErrorKind , _e : E) -> Self { Error { input , code : kind } } }
};
}
