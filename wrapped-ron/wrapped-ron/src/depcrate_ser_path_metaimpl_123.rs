// Generated macro for impl_123 (impl)
macro_rules! Depcrate_ser_path_metaimpl_123 {
() => {
// Module: crate::ser::path_meta
// Provides: {"impl_123"}
// Dependencies: {}
impl < K : Into < String > > FromIterator < (K , Field) > for Fields { fn from_iter < T : IntoIterator < Item = (K , Field) > > (iter : T) -> Self { Self { fields : iter . into_iter () . map (| (k , v) | (k . into () , v)) . collect () , } } }
};
}
