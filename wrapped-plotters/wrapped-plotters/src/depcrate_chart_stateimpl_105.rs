// Generated macro for impl_105 (impl)
macro_rules! Depcrate_chart_stateimpl_105 {
() => {
// Module: crate::chart::state
// Provides: {"impl_105"}
// Dependencies: {}
impl < 'a , DB : DrawingBackend , CT : CoordTranslate > ChartContext < 'a , DB , CT > { # [doc = " Convert a chart context into a chart state, by doing so, the chart context is consumed and"] # [doc = " a saved chart state is created for later use. This is typically used in incremental rendering. See documentation of `ChartState` for more detailed example."] pub fn into_chart_state (self) -> ChartState < CT > { self . into () } # [doc = " Convert the chart context into a sharable chart state."] # [doc = " Normally a chart state can not be clone, since the coordinate spec may not be able to be"] # [doc = " cloned. In this case, we can use an `Arc` get the coordinate wrapped thus the state can be"] # [doc = " cloned and shared by multiple chart context"] pub fn into_shared_chart_state (self) -> ChartState < Arc < CT > > { ChartState { drawing_area_pos : self . drawing_area_pos , drawing_area_size : self . drawing_area . dim_in_pixel () , coord : Arc :: new (self . drawing_area . into_coord_spec ()) , } } }
};
}
