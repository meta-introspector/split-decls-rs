// Generated macro for impl_290 (impl)
macro_rules! Depcrate_valueimpl_290 {
() => {
// Module: crate::value
// Provides: {"impl_290"}
// Dependencies: {}
impl < K : Into < Value > , V : Into < Value > > FromIterator < (K , V) > for Value { fn from_iter < T : IntoIterator < Item = (K , V) > > (iter : T) -> Self { Self :: Map (iter . into_iter () . collect ()) } }
};
}
