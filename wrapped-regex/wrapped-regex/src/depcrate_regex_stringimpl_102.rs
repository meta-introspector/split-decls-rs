// Generated macro for impl_102 (impl)
macro_rules! Depcrate_regex_stringimpl_102 {
() => {
// Module: crate::regex::string
// Provides: {"impl_102"}
// Dependencies: {}
impl core :: str :: FromStr for Regex { type Err = Error ; # [doc = " Attempts to parse a string into a regular expression"] fn from_str (s : & str) -> Result < Regex , Error > { Regex :: new (s) } }
};
}
