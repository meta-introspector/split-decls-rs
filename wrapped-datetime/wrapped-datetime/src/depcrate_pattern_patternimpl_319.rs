// Generated macro for impl_319 (impl)
macro_rules! Depcrate_pattern_patternimpl_319 {
() => {
// Module: crate::pattern::pattern
// Provides: {"impl_319"}
// Dependencies: {}
impl DateTimePattern { # [doc = " Creates a [`DateTimePattern`] from a pattern string."] # [doc = ""] # [doc = " For more details on the syntax, see UTS 35:"] # [doc = " <https://unicode.org/reports/tr35/tr35-dates.html#Date_Format_Patterns>"] pub fn try_from_pattern_str (pattern_str : & str) -> Result < Self , PatternError > { let pattern = runtime :: Pattern :: from_str (pattern_str) ? ; Ok (Self { pattern }) } pub (crate) fn iter_items (& self) -> impl Iterator < Item = PatternItem > + '_ { self . pattern . items . iter () } pub (crate) fn as_borrowed (& self) -> DateTimePatternBorrowed < '_ > { DateTimePatternBorrowed (& self . pattern) } }
};
}
