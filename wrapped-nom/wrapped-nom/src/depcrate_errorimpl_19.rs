// Generated macro for impl_19 (impl)
macro_rules! Depcrate_errorimpl_19 {
() => {
// Module: crate::error
// Provides: {"impl_19"}
// Dependencies: {}
impl < I > ParseError < I > for Error < I > { fn from_error_kind (input : I , kind : ErrorKind) -> Self { Error { input , code : kind } } fn append (_ : I , _ : ErrorKind , other : Self) -> Self { other } }
};
}
