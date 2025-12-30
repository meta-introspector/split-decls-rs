// Generated macro for impl_104 (impl)
macro_rules! Depcrate_regex_stringimpl_104 {
() => {
// Module: crate::regex::string
// Provides: {"impl_104"}
// Dependencies: {}
impl TryFrom < String > for Regex { type Error = Error ; # [doc = " Attempts to parse a string into a regular expression"] fn try_from (s : String) -> Result < Regex , Error > { Regex :: new (& s) } }
};
}
