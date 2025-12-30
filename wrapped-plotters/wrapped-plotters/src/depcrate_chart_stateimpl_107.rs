// Generated macro for impl_107 (impl)
macro_rules! Depcrate_chart_stateimpl_107 {
() => {
// Module: crate::chart::state
// Provides: {"impl_107"}
// Dependencies: {}
impl < 'a , DB : DrawingBackend , CT : CoordTranslate + Clone > ChartContext < 'a , DB , CT > { # [doc = " Make the chart context, do not consume the chart context and clone the coordinate spec"] pub fn to_chart_state (& self) -> ChartState < CT > { self . into () } }
};
}
