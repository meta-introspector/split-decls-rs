// Generated macro for impl_159 (impl)
macro_rules! Depcrate_header_nameimpl_159 {
() => {
// Module: crate::header::name
// Provides: {"impl_159"}
// Dependencies: {}
impl FromStr for HeaderName { type Err = InvalidHeaderName ; fn from_str (s : & str) -> Result < HeaderName , InvalidHeaderName > { HeaderName :: from_bytes (s . as_bytes ()) . map_err (| _ | InvalidHeaderName { _priv : () }) } }
};
}
