// Generated macro for impl_128 (impl)
macro_rules! Depcrate_parse_typesimpl_128 {
() => {
// Module: crate::parse::types
// Provides: {"impl_128"}
// Dependencies: {}
impl < 'a , E > FromExternalError < Input < 'a > , E > for Error < 'a > { fn from_external_error (input : Input < 'a > , kind : ErrorKind , _e : E) -> Self { Error { input , code : kind , detail : None } } }
};
}
