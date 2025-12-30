// Generated macro for ChartBuilder (struct)
macro_rules! Depcrate_chart_builderChartBuilder {
() => {
// Module: crate::chart::builder
// Provides: {"ChartBuilder"}
// Dependencies: {}
# [doc = "\nThe helper object to create a chart context, which is used for the high-level figure drawing.\n\nWith the help of this object, we can convert a basic drawing area into a chart context, which\nallows the high-level charting API being used on the drawing area.\n\nSee [`ChartBuilder::on()`] for more information and examples.\n"] pub struct ChartBuilder < 'a , 'b , DB : DrawingBackend > { label_area_size : [u32 ; 4] , overlap_plotting_area : [bool ; 4] , root_area : & 'a DrawingArea < DB , Shift > , title : Option < (String , TextStyle < 'b >) > , margin : [u32 ; 4] , }
};
}
