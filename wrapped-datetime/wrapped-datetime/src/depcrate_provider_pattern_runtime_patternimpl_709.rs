// Generated macro for impl_709 (impl)
macro_rules! Depcrate_provider_pattern_runtime_patternimpl_709 {
() => {
// Module: crate::provider::pattern::runtime::pattern
// Provides: {"impl_709"}
// Dependencies: {}
impl Pattern < '_ > { pub (crate) fn into_owned (self) -> Pattern < 'static > { Pattern { items : self . items . into_owned () , metadata : self . metadata , } } pub (crate) fn as_borrowed (& self) -> PatternBorrowed < '_ > { PatternBorrowed { items : & self . items , metadata : self . metadata , } } # [doc = " Borrows a [`Pattern`] from another [`Pattern`]."] pub fn as_ref (& self) -> Pattern < '_ > { self . as_borrowed () . as_pattern () } }
};
}
