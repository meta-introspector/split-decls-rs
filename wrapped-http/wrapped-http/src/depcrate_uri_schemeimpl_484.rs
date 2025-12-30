// Generated macro for impl_484 (impl)
macro_rules! Depcrate_uri_schemeimpl_484 {
() => {
// Module: crate::uri::scheme
// Provides: {"impl_484"}
// Dependencies: {}
impl FromStr for Scheme { type Err = InvalidUri ; fn from_str (s : & str) -> Result < Self , Self :: Err > { TryFrom :: try_from (s) } }
};
}
