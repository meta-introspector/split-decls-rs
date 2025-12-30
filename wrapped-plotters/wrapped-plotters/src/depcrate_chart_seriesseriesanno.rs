// Generated macro for SeriesAnno (struct)
macro_rules! Depcrate_chart_seriesSeriesAnno {
() => {
// Module: crate::chart::series
// Provides: {"SeriesAnno"}
// Dependencies: {}
# [doc = " The annotations (such as the label of the series, the legend element, etc)"] # [doc = " When a series is drawn onto a drawing area, an series annotation object"] # [doc = " is created and a mutable reference is returned."] pub struct SeriesAnno < 'a , DB : DrawingBackend > { label : Option < String > , draw_func : Option < Box < SeriesAnnoDrawFn < 'a , DB > > > , }
};
}
