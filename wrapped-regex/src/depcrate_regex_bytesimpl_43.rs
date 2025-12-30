// Generated macro for impl_43 (impl)
macro_rules! Depcrate_regex_bytesimpl_43 {
() => {
// Module: crate::regex::bytes
// Provides: {"impl_43"}
// Dependencies: {}
impl TryFrom < String > for Regex { type Error = Error ; # [doc = " Attempts to parse a string into a regular expression"] fn try_from (s : String) -> Result < Regex , Error > { Regex :: new (& s) } }
};
}
