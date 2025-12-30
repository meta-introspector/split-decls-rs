// Generated macro for impl_218 (impl)
macro_rules! Depcrate_header_valueimpl_218 {
() => {
// Module: crate::header::value
// Provides: {"impl_218"}
// Dependencies: {}
impl FromStr for HeaderValue { type Err = InvalidHeaderValue ; # [inline] fn from_str (s : & str) -> Result < HeaderValue , Self :: Err > { HeaderValue :: from_str (s) } }
};
}
