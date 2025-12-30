// Generated macro for impl_406 (impl)
macro_rules! Depcrate_uri_authorityimpl_406 {
() => {
// Module: crate::uri::authority
// Provides: {"impl_406"}
// Dependencies: {}
impl FromStr for Authority { type Err = InvalidUri ; fn from_str (s : & str) -> Result < Self , InvalidUri > { TryFrom :: try_from (s) } }
};
}
