// Generated macro for impl_298 (impl)
macro_rules! Depcrate_re_bytesimpl_298 {
() => {
// Module: crate::re_bytes
// Provides: {"impl_298"}
// Dependencies: {}
impl FromStr for Regex { type Err = Error ; # [doc = " Attempts to parse a string into a regular expression"] fn from_str (s : & str) -> Result < Regex , Error > { Regex :: new (s) } }
};
}
