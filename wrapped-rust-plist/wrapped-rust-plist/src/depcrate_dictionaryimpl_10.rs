// Generated macro for impl_10 (impl)
macro_rules! Depcrate_dictionaryimpl_10 {
() => {
// Module: crate::dictionary
// Provides: {"impl_10"}
// Dependencies: {}
impl < K : Into < String > , V : Into < Value > > FromIterator < (K , V) > for Dictionary { fn from_iter < I : IntoIterator < Item = (K , V) > > (iter : I) -> Self { Dictionary { map : iter . into_iter () . map (| (k , v) | (k . into () , v . into ())) . collect () , } } }
};
}
