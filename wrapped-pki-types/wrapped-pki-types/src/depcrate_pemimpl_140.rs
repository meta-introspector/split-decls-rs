// Generated macro for impl_140 (impl)
macro_rules! Depcrate_pemimpl_140 {
() => {
// Module: crate::pem
// Provides: {"impl_140"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: MissingSectionEnd { end_marker } => { write ! (f , "missing section end marker: {end_marker:?}") } Self :: IllegalSectionStart { line } => { write ! (f , "illegal section start: {line:?}") } Self :: Base64Decode (e) => write ! (f , "base64 decode error: {e}") , # [cfg (feature = "std")] Self :: Io (e) => write ! (f , "I/O error: {e}") , Self :: NoItemsFound => write ! (f , "no items found") , } } }
};
}
