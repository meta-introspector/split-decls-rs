// Generated macro for SeriesLabelStyle (struct)
macro_rules! Depcrate_chart_seriesSeriesLabelStyle {
() => {
// Module: crate::chart::series
// Provides: {"SeriesLabelStyle"}
// Dependencies: {}
# [doc = " The struct to specify the series label of a target chart context"] pub struct SeriesLabelStyle < 'a , 'b , DB : DrawingBackend , CT : CoordTranslate > { target : & 'b mut ChartContext < 'a , DB , CT > , position : SeriesLabelPosition , legend_area_size : u32 , border_style : ShapeStyle , background : ShapeStyle , label_font : Option < TextStyle < 'b > > , margin : u32 , }
};
}
