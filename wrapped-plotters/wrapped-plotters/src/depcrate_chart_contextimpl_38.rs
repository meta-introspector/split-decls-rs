// Generated macro for impl_38 (impl)
macro_rules! Depcrate_chart_contextimpl_38 {
() => {
// Module: crate::chart::context
// Provides: {"impl_38"}
// Dependencies: {}
impl < 'a , DB : DrawingBackend , CT : ReverseCoordTranslate > ChartContext < 'a , DB , CT > { # [doc = " Convert the chart context into an closure that can be used for coordinate translation"] pub fn into_coord_trans (self) -> impl Fn (BackendCoord) -> Option < CT :: From > { let coord_spec = self . drawing_area . into_coord_spec () ; move | coord | coord_spec . reverse_translate (coord) } }
};
}
