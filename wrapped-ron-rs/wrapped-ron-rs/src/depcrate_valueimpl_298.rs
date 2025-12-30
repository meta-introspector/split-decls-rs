// Generated macro for impl_298 (impl)
macro_rules! Depcrate_valueimpl_298 {
() => {
// Module: crate::value
// Provides: {"impl_298"}
// Dependencies: {}
impl < T : Into < Value > > FromIterator < T > for Value { fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> Self { Self :: Seq (iter . into_iter () . map (Into :: into) . collect ()) } }
};
}
