// Generated macro for impl_191 (impl)
macro_rules! Depcrate_stringimpl_191 {
() => {
// Module: crate::string
// Provides: {"impl_191"}
// Dependencies: {}
impl TryFrom < & str > for Regex { type Error = Error ; # [doc = " Attempts to parse a string into a regular expression"] fn try_from (s : & str) -> Result < Regex , Error > { Regex :: new (s) } }
};
}
