// Generated macro for impl_496 (impl)
macro_rules! Depcrateimpl_496 {
() => {
// Module: crate
// Provides: {"impl_496"}
// Dependencies: {}
impl FromIterator < Box < str > > for CompactString { fn from_iter < T : IntoIterator < Item = Box < str > > > (iter : T) -> Self { let repr = iter . into_iter () . collect () ; CompactString (repr) } }
};
}
