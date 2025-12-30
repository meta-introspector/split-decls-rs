// Generated macro for impl_203 (impl)
macro_rules! Depcrate_neoimpl_203 {
() => {
// Module: crate::neo
// Provides: {"impl_203"}
// Dependencies: {}
impl FormattedDateTime < '_ > { # [doc = " Gets the pattern used in this formatted value."] # [doc = ""] # [doc = " From the pattern, one can check the properties of the included components, such as"] # [doc = " the hour cycle being used for formatting. See [`DateTimePattern`]."] pub fn pattern (& self) -> DateTimePattern { self . pattern . to_pattern () } }
};
}
