// Generated macro for impl_1228 (impl)
macro_rules! Depcrate_uncheckedimpl_1228 {
() => {
// Module: crate::unchecked
// Provides: {"impl_1228"}
// Dependencies: {}
impl FormattedDateTimeUnchecked < '_ > { # [doc = " Gets the pattern used in this formatted value."] # [doc = ""] # [doc = " From the pattern, one can check the properties of the included components, such as"] # [doc = " the hour cycle being used for formatting. See [`DateTimePattern`]."] pub fn pattern (& self) -> DateTimePattern { self . pattern . to_pattern () } }
};
}
