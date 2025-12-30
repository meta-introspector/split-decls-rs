// Generated macro for impl_562 (impl)
macro_rules! Depcrate_uriimpl_562 {
() => {
// Module: crate::uri
// Provides: {"impl_562"}
// Dependencies: {}
impl FromStr for Uri { type Err = InvalidUri ; # [inline] fn from_str (s : & str) -> Result < Uri , InvalidUri > { Uri :: try_from (s . as_bytes ()) } }
};
}
