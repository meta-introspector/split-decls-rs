// Generated macro for impl_71 (impl)
macro_rules! Depcrate_decodeimpl_71 {
() => {
// Module: crate::decode
// Provides: {"impl_71"}
// Dependencies: {}
impl From < DecodeStringError < '_ > > for Error { # [cold] fn from (err : DecodeStringError < '_ >) -> Self { match err { DecodeStringError :: InvalidMarkerRead (err) => Self :: InvalidMarkerRead (err) , DecodeStringError :: InvalidDataRead (err) => Self :: InvalidDataRead (err) , DecodeStringError :: TypeMismatch (marker) => Self :: TypeMismatch (marker) , DecodeStringError :: BufferSizeTooSmall (..) => Self :: Uncategorized ("BufferSizeTooSmall" . to_string ()) , DecodeStringError :: InvalidUtf8 (..) => Self :: Uncategorized ("InvalidUtf8" . to_string ()) , } } }
};
}
