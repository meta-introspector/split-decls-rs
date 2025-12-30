// Generated macro for impl_633 (impl)
macro_rules! Depcrate_layoutimpl_633 {
() => {
// Module: crate::layout
// Provides: {"impl_633"}
// Dependencies: {}
impl < F > From < LayoutCalculatorError < F > > for LayoutError { fn from (err : LayoutCalculatorError < F >) -> Self { LayoutError :: BadCalc (err . without_payload ()) } }
};
}
