// Generated macro for impl_104 (impl)
macro_rules! Depcrate_chart_stateimpl_104 {
() => {
// Module: crate::chart::state
// Provides: {"impl_104"}
// Dependencies: {}
impl < 'a , DB : DrawingBackend , CT : CoordTranslate > From < ChartContext < 'a , DB , CT > > for ChartState < CT > { fn from (chart : ChartContext < 'a , DB , CT >) -> ChartState < CT > { ChartState { drawing_area_pos : chart . drawing_area_pos , drawing_area_size : chart . drawing_area . dim_in_pixel () , coord : chart . drawing_area . into_coord_spec () , } } }
};
}
