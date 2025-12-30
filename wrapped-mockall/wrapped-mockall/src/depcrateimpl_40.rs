// Generated macro for impl_40 (impl)
macro_rules! Depcrateimpl_40 {
() => {
// Module: crate
// Provides: {"impl_40"}
// Dependencies: {}
impl From < RangeToInclusive < usize > > for TimesRange { fn from (r : RangeToInclusive < usize >) -> TimesRange { TimesRange (0 .. r . end + 1) } }
};
}
