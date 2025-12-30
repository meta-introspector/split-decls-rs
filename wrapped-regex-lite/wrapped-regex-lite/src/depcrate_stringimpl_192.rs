// Generated macro for impl_192 (impl)
macro_rules! Depcrate_stringimpl_192 {
() => {
// Module: crate::string
// Provides: {"impl_192"}
// Dependencies: {}
impl TryFrom < String > for Regex { type Error = Error ; # [doc = " Attempts to parse a string into a regular expression"] fn try_from (s : String) -> Result < Regex , Error > { Regex :: new (& s) } }
};
}
