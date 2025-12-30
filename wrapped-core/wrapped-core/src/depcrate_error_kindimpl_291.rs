// Generated macro for impl_291 (impl)
macro_rules! Depcrate_error_kindimpl_291 {
() => {
// Module: crate::error::kind
// Provides: {"impl_291"}
// Dependencies: {}
impl From < ErrorUnknownValue > for ErrorKind { fn from (value : ErrorUnknownValue) -> Self { match value . noun { UnknownValuePosition :: Field => Self :: UnknownField (Box :: new (value)) , UnknownValuePosition :: Value => Self :: UnknownValue (Box :: new (value)) , } } }
};
}
