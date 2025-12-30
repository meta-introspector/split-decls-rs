// Generated macro for impl_190 (impl)
macro_rules! Depcrate_stringimpl_190 {
() => {
// Module: crate::string
// Provides: {"impl_190"}
// Dependencies: {}
impl core :: str :: FromStr for Regex { type Err = Error ; # [doc = " Attempts to parse a string into a regular expression"] fn from_str (s : & str) -> Result < Regex , Error > { Regex :: new (s) } }
};
}
