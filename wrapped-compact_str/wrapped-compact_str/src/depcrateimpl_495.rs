// Generated macro for impl_495 (impl)
macro_rules! Depcrateimpl_495 {
() => {
// Module: crate
// Provides: {"impl_495"}
// Dependencies: {}
impl < 'a > FromIterator < & 'a str > for CompactString { fn from_iter < T : IntoIterator < Item = & 'a str > > (iter : T) -> Self { let repr = iter . into_iter () . collect () ; CompactString (repr) } }
};
}
