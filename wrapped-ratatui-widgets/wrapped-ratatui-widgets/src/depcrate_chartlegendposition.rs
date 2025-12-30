// Generated macro for LegendPosition (enum)
macro_rules! Depcrate_chartLegendPosition {
() => {
// Module: crate::chart
// Provides: {"LegendPosition"}
// Dependencies: {}
# [doc = " Allow users to specify the position of a legend in a [`Chart`]"] # [doc = ""] # [doc = " See [`Chart::legend_position`]"] # [derive (Debug , Default , Clone , Copy , Eq , PartialEq)] pub enum LegendPosition { # [doc = " Legend is centered on top"] Top , # [doc = " Legend is in the top-right corner. This is the **default**."] # [default] TopRight , # [doc = " Legend is in the top-left corner"] TopLeft , # [doc = " Legend is centered on the left"] Left , # [doc = " Legend is centered on the right"] Right , # [doc = " Legend is centered on the bottom"] Bottom , # [doc = " Legend is in the bottom-right corner"] BottomRight , # [doc = " Legend is in the bottom-left corner"] BottomLeft , }
};
}
