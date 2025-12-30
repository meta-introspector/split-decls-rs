// Generated macro for impl_32 (impl)
macro_rules! Depcrate_errorimpl_32 {
() => {
// Module: crate::error
// Provides: {"impl_32"}
// Dependencies: {}
impl < I , E > FromExternalError < I , E > for (I , ErrorKind) { fn from_external_error (input : I , kind : ErrorKind , _e : E) -> Self { (input , kind) } }
};
}
