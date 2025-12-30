// Generated macro for impl_8 (impl)
macro_rules! Depcrate_errorimpl_8 {
() => {
// Module: crate::error
// Provides: {"impl_8"}
// Dependencies: {}
impl < I , E > FromExternalError < I , E > for VerboseError < I > { # [doc = " Create a new error from an input position and an external error"] fn from_external_error (input : I , kind : ErrorKind , _e : E) -> Self { Self :: from_error_kind (input , kind) } }
};
}
