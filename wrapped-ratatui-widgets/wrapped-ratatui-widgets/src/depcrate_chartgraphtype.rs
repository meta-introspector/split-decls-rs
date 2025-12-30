// Generated macro for GraphType (enum)
macro_rules! Depcrate_chartGraphType {
() => {
// Module: crate::chart
// Provides: {"GraphType"}
// Dependencies: {}
# [doc = " Used to determine which style of graphing to use"] # [derive (Debug , Default , Display , EnumString , Clone , Copy , Eq , PartialEq , Hash)] pub enum GraphType { # [doc = " Draw each point. This is the default."] # [default] Scatter , # [doc = " Draw a line between each following point."] # [doc = ""] # [doc = " The order of the lines will be the same as the order of the points in the dataset, which"] # [doc = " allows this widget to draw lines both left-to-right and right-to-left"] Line , # [doc = " Draw a bar chart. This will draw a bar for each point in the dataset."] Bar , }
};
}
