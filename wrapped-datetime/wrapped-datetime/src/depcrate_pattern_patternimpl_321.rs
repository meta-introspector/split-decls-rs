// Generated macro for impl_321 (impl)
macro_rules! Depcrate_pattern_patternimpl_321 {
() => {
// Module: crate::pattern::pattern
// Provides: {"impl_321"}
// Dependencies: {}
impl < 'a > From < runtime :: PatternBorrowed < 'a > > for DateTimePattern { fn from (pattern : runtime :: PatternBorrowed < 'a >) -> Self { Self { pattern : pattern . as_pattern () . into_owned () , } } }
};
}
