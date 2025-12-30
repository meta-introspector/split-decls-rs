// Generated macro for impl_127 (impl)
macro_rules! Depcrate_parse_typesimpl_127 {
() => {
// Module: crate::parse::types
// Provides: {"impl_127"}
// Dependencies: {}
# [doc = " Mandatory [`ParseError`] implementation."] impl < 'a > ParseError < Input < 'a > > for Error < 'a > { fn from_error_kind (input : Input < 'a > , kind : ErrorKind) -> Self { Error { input , code : kind , detail : None } } fn append (_ : Input < 'a > , _ : ErrorKind , other : Self) -> Self { other } }
};
}
