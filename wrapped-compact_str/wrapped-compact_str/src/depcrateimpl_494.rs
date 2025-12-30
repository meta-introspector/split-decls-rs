// Generated macro for impl_494 (impl)
macro_rules! Depcrateimpl_494 {
() => {
// Module: crate
// Provides: {"impl_494"}
// Dependencies: {}
impl < 'a > FromIterator < & 'a char > for CompactString { fn from_iter < T : IntoIterator < Item = & 'a char > > (iter : T) -> Self { let repr = iter . into_iter () . collect () ; CompactString (repr) } }
};
}
