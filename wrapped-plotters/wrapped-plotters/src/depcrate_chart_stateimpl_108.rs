// Generated macro for impl_108 (impl)
macro_rules! Depcrate_chart_stateimpl_108 {
() => {
// Module: crate::chart::state
// Provides: {"impl_108"}
// Dependencies: {}
impl < CT : CoordTranslate > ChartState < CT > { # [doc = " Restore the chart context on the given drawing area"] # [doc = ""] # [doc = " - `area`: The given drawing area where we want to restore the chart context"] # [doc = " - **returns** The newly created chart context"] pub fn restore < 'a , DB : DrawingBackend > (self , area : & DrawingArea < DB , Shift > ,) -> ChartContext < 'a , DB , CT > { let area = area . clone () . shrink (self . drawing_area_pos , self . drawing_area_size) ; ChartContext { x_label_area : [None , None] , y_label_area : [None , None] , drawing_area : area . apply_coord_spec (self . coord) , series_anno : vec ! [] , drawing_area_pos : self . drawing_area_pos , } } }
};
}
