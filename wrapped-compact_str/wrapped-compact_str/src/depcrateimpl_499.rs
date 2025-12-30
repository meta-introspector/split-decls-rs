// Generated macro for impl_499 (impl)
macro_rules! Depcrateimpl_499 {
() => {
// Module: crate
// Provides: {"impl_499"}
// Dependencies: {}
impl FromIterator < CompactString > for CompactString { fn from_iter < T : IntoIterator < Item = CompactString > > (iter : T) -> Self { let repr = iter . into_iter () . collect () ; CompactString (repr) } }
};
}
