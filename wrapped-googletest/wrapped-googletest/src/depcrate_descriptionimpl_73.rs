// Generated macro for impl_73 (impl)
macro_rules! Depcrate_descriptionimpl_73 {
() => {
// Module: crate::description
// Provides: {"impl_73"}
// Dependencies: {}
impl FromIterator < Description > for Description { fn from_iter < T > (iter : T) -> Self where T : IntoIterator < Item = Description > , { Self { elements : iter . into_iter () . map (| s | s . elements) . collect () , .. Default :: default () } } }
};
}
