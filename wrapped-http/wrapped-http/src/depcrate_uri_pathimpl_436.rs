// Generated macro for impl_436 (impl)
macro_rules! Depcrate_uri_pathimpl_436 {
() => {
// Module: crate::uri::path
// Provides: {"impl_436"}
// Dependencies: {}
impl FromStr for PathAndQuery { type Err = InvalidUri ; # [inline] fn from_str (s : & str) -> Result < Self , InvalidUri > { TryFrom :: try_from (s) } }
};
}
