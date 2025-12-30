// Generated macro for impl_96 (impl)
macro_rules! Depcrate_internal_description_rendererimpl_96 {
() => {
// Module: crate::internal::description_renderer
// Provides: {"impl_96"}
// Dependencies: {}
impl < ElementT : Into < Cow < 'static , str > > > FromIterator < ElementT > for List { fn from_iter < T > (iter : T) -> Self where T : IntoIterator < Item = ElementT > , { Self (iter . into_iter () . map (| b | b . into () . into ()) . collect () , Decoration :: None) } }
};
}
