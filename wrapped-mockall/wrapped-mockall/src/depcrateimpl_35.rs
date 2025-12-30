// Generated macro for impl_35 (impl)
macro_rules! Depcrateimpl_35 {
() => {
// Module: crate
// Provides: {"impl_35"}
// Dependencies: {}
impl From < Range < usize > > for TimesRange { fn from (r : Range < usize >) -> TimesRange { assert ! (r . end > r . start , "Backwards range") ; TimesRange (r) } }
};
}
