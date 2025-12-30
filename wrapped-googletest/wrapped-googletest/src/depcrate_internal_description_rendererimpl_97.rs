// Generated macro for impl_97 (impl)
macro_rules! Depcrate_internal_description_rendererimpl_97 {
() => {
// Module: crate::internal::description_renderer
// Provides: {"impl_97"}
// Dependencies: {}
impl FromIterator < List > for List { fn from_iter < T > (iter : T) -> Self where T : IntoIterator < Item = List > , { Self (iter . into_iter () . map (Block :: nested) . collect () , Decoration :: None) } }
};
}
