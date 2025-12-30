// Generated macro for impl_103 (impl)
macro_rules! Depcrate_regex_stringimpl_103 {
() => {
// Module: crate::regex::string
// Provides: {"impl_103"}
// Dependencies: {}
impl TryFrom < & str > for Regex { type Error = Error ; # [doc = " Attempts to parse a string into a regular expression"] fn try_from (s : & str) -> Result < Regex , Error > { Regex :: new (s) } }
};
}
