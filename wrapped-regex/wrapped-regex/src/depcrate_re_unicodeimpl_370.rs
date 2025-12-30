// Generated macro for impl_370 (impl)
macro_rules! Depcrate_re_unicodeimpl_370 {
() => {
// Module: crate::re_unicode
// Provides: {"impl_370"}
// Dependencies: {}
impl FromStr for Regex { type Err = Error ; # [doc = " Attempts to parse a string into a regular expression"] fn from_str (s : & str) -> Result < Regex , Error > { Regex :: new (s) } }
};
}
