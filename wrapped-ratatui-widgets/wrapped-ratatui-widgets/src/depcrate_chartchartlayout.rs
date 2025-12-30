// Generated macro for ChartLayout (struct)
macro_rules! Depcrate_chartChartLayout {
() => {
// Module: crate::chart
// Provides: {"ChartLayout"}
// Dependencies: {}
# [doc = " A container that holds all the infos about where to display each elements of the chart (axis,"] # [doc = " labels, legend, ...)."] struct ChartLayout { # [doc = " Location of the title of the x axis"] title_x : Option < Position > , # [doc = " Location of the title of the y axis"] title_y : Option < Position > , # [doc = " Location of the first label of the x axis"] label_x : Option < u16 > , # [doc = " Location of the first label of the y axis"] label_y : Option < u16 > , # [doc = " Y coordinate of the horizontal axis"] axis_x : Option < u16 > , # [doc = " X coordinate of the vertical axis"] axis_y : Option < u16 > , # [doc = " Area of the legend"] legend_area : Option < Rect > , # [doc = " Area of the graph"] graph_area : Rect , }
};
}
