// Generated macro for impl_72 (impl)
macro_rules! Depcrate_descriptionimpl_72 {
() => {
// Module: crate::description
// Provides: {"impl_72"}
// Dependencies: {}
impl < ElementT : Into < Cow < 'static , str > > > FromIterator < ElementT > for Description { fn from_iter < T > (iter : T) -> Self where T : IntoIterator < Item = ElementT > , { Self { elements : iter . into_iter () . map (ElementT :: into) . collect () , .. Default :: default () } } }
};
}
