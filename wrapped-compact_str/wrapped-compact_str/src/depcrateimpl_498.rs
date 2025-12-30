// Generated macro for impl_498 (impl)
macro_rules! Depcrateimpl_498 {
() => {
// Module: crate
// Provides: {"impl_498"}
// Dependencies: {}
impl FromIterator < String > for CompactString { fn from_iter < T : IntoIterator < Item = String > > (iter : T) -> Self { let repr = iter . into_iter () . collect () ; CompactString (repr) } }
};
}
