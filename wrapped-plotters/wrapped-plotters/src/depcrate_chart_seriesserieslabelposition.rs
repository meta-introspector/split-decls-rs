// Generated macro for SeriesLabelPosition (enum)
macro_rules! Depcrate_chart_seriesSeriesLabelPosition {
() => {
// Module: crate::chart::series
// Provides: {"SeriesLabelPosition"}
// Dependencies: {}
# [doc = "\nUseful to specify the position of the series label.\n\nSee [`ChartContext::configure_series_labels()`] for more information and examples.\n"] # [derive (Debug , Clone , PartialEq)] pub enum SeriesLabelPosition { # [doc = " Places the series label at the upper left"] UpperLeft , # [doc = " Places the series label at the middle left"] MiddleLeft , # [doc = " Places the series label at the lower left"] LowerLeft , # [doc = " Places the series label at the upper middle"] UpperMiddle , # [doc = " Places the series label at the middle middle"] MiddleMiddle , # [doc = " Places the series label at the lower middle"] LowerMiddle , # [doc = " Places the series label at the upper right"] UpperRight , # [doc = " Places the series label at the middle right"] MiddleRight , # [doc = " Places the series label at the lower right"] LowerRight , # [doc = " Places the series label at the specific location in backend coordinates"] Coordinate (i32 , i32) , }
};
}
