// Generated macro for impl_293 (impl)
macro_rules! Depcrate_valueimpl_293 {
() => {
// Module: crate::value
// Provides: {"impl_293"}
// Dependencies: {}
impl < T : Into < Value > > From < Option < T > > for Value { fn from (value : Option < T >) -> Self { Self :: Option (value . map (Into :: into) . map (Box :: new)) } }
};
}
