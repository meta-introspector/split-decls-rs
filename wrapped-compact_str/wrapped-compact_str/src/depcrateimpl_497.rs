// Generated macro for impl_497 (impl)
macro_rules! Depcrateimpl_497 {
() => {
// Module: crate
// Provides: {"impl_497"}
// Dependencies: {}
impl < 'a > FromIterator < Cow < 'a , str > > for CompactString { fn from_iter < T : IntoIterator < Item = Cow < 'a , str > > > (iter : T) -> Self { let repr = iter . into_iter () . collect () ; CompactString (repr) } }
};
}
