// Generated macro for impl_715 (impl)
macro_rules! Depcrate_layoutimpl_715 {
() => {
// Module: crate::layout
// Provides: {"impl_715"}
// Dependencies: {}
impl < F > From < LayoutCalculatorError < F > > for LayoutError { fn from (err : LayoutCalculatorError < F >) -> Self { LayoutError :: BadCalc (err . without_payload ()) } }
};
}
