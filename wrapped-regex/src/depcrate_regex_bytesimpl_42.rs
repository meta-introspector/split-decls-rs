// Generated macro for impl_42 (impl)
macro_rules! Depcrate_regex_bytesimpl_42 {
() => {
// Module: crate::regex::bytes
// Provides: {"impl_42"}
// Dependencies: {}
impl TryFrom < & str > for Regex { type Error = Error ; # [doc = " Attempts to parse a string into a regular expression"] fn try_from (s : & str) -> Result < Regex , Error > { Regex :: new (s) } }
};
}
