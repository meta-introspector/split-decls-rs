// Generated macro for impl_349 (impl)
macro_rules! Depcrate_statusimpl_349 {
() => {
// Module: crate::status
// Provides: {"impl_349"}
// Dependencies: {}
impl FromStr for StatusCode { type Err = InvalidStatusCode ; fn from_str (s : & str) -> Result < StatusCode , InvalidStatusCode > { StatusCode :: from_bytes (s . as_ref ()) } }
};
}
