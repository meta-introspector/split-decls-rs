// Generated macro for impl_66 (impl)
macro_rules! Depcrate_decodeimpl_66 {
() => {
// Module: crate::decode
// Provides: {"impl_66"}
// Dependencies: {}
impl Display for Error { # [cold] fn fmt (& self , fmt : & mut Formatter < '_ >) -> Result < () , fmt :: Error > { match * self { Self :: InvalidMarkerRead (ref err) => write ! (fmt , "IO error while reading marker: {err}") , Self :: InvalidDataRead (ref err) => write ! (fmt , "IO error while reading data: {err}") , Self :: TypeMismatch (ref actual_marker) => { write ! (fmt , "wrong msgpack marker {actual_marker:?}") } Self :: OutOfRange => fmt . write_str ("numeric cast found out of range") , Self :: LengthMismatch (expected_length) => write ! (fmt , "array had incorrect length, expected {expected_length}") , Self :: Uncategorized (ref msg) => write ! (fmt , "uncategorized error: {msg}") , Self :: Syntax (ref msg) => fmt . write_str (msg) , Self :: Utf8Error (ref err) => write ! (fmt , "string found to be invalid utf8: {err}") , Self :: DepthLimitExceeded => fmt . write_str ("depth limit exceeded") , } } }
};
}
