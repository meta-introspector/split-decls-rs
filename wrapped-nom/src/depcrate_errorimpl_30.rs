// Generated macro for impl_30 (impl)
macro_rules! Depcrate_errorimpl_30 {
() => {
// Module: crate::error
// Provides: {"impl_30"}
// Dependencies: {}
impl < I > ParseError < I > for (I , ErrorKind) { fn from_error_kind (input : I , kind : ErrorKind) -> Self { (input , kind) } fn append (_ : I , _ : ErrorKind , other : Self) -> Self { other } }
};
}
