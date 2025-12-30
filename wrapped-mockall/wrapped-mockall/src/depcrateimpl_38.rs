// Generated macro for impl_38 (impl)
macro_rules! Depcrateimpl_38 {
() => {
// Module: crate
// Provides: {"impl_38"}
// Dependencies: {}
impl From < RangeInclusive < usize > > for TimesRange { fn from (r : RangeInclusive < usize >) -> TimesRange { assert ! (r . end () >= r . start () , "Backwards range") ; TimesRange (* r . start () .. * r . end () + 1) } }
};
}
