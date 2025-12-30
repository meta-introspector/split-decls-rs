// Generated macro for impl_493 (impl)
macro_rules! Depcrateimpl_493 {
() => {
// Module: crate
// Provides: {"impl_493"}
// Dependencies: {}
impl FromIterator < char > for CompactString { fn from_iter < T : IntoIterator < Item = char > > (iter : T) -> Self { let repr = iter . into_iter () . collect () ; CompactString (repr) } }
};
}
