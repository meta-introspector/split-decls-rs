// Generated macro for impl_106 (impl)
macro_rules! Depcrate_chart_stateimpl_106 {
() => {
// Module: crate::chart::state
// Provides: {"impl_106"}
// Dependencies: {}
impl < 'a , DB , CT > From < & ChartContext < 'a , DB , CT > > for ChartState < CT > where DB : DrawingBackend , CT : CoordTranslate + Clone , { fn from (chart : & ChartContext < 'a , DB , CT >) -> ChartState < CT > { ChartState { drawing_area_pos : chart . drawing_area_pos , drawing_area_size : chart . drawing_area . dim_in_pixel () , coord : chart . drawing_area . as_coord_spec () . clone () , } } }
};
}
