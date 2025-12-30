// Generated macro for impl_234 (impl)
macro_rules! Depcrate_value_mapimpl_234 {
() => {
// Module: crate::value::map
// Provides: {"impl_234"}
// Dependencies: {}
impl < K : Into < Value > , V : Into < Value > > FromIterator < (K , V) > for Map { fn from_iter < T : IntoIterator < Item = (K , V) > > (iter : T) -> Self { Map (iter . into_iter () . map (| (key , value) | (key . into () , value . into ())) . collect ()) } }
};
}
