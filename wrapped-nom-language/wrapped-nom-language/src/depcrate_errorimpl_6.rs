// Generated macro for impl_6 (impl)
macro_rules! Depcrate_errorimpl_6 {
() => {
// Module: crate::error
// Provides: {"impl_6"}
// Dependencies: {}
impl < I > ParseError < I > for VerboseError < I > { fn from_error_kind (input : I , kind : ErrorKind) -> Self { VerboseError { errors : vec ! [(input , VerboseErrorKind :: Nom (kind))] , } } fn append (input : I , kind : ErrorKind , mut other : Self) -> Self { other . errors . push ((input , VerboseErrorKind :: Nom (kind))) ; other } fn from_char (input : I , c : char) -> Self { VerboseError { errors : vec ! [(input , VerboseErrorKind :: Char (c))] , } } }
};
}
