// Generated macro for impl_385 (impl)
macro_rules! Depcrate_builder_value_parserimpl_385 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_385"}
// Dependencies: {}
impl < I , T > From < I > for PossibleValuesParser where I : IntoIterator < Item = T > , T : Into < super :: PossibleValue > , { fn from (values : I) -> Self { Self (values . into_iter () . map (| t | t . into ()) . collect ()) } }
};
}
