// Generated macro for Axis (struct)
macro_rules! Depcrate_chartAxis {
() => {
// Module: crate::chart
// Provides: {"Axis"}
// Dependencies: {}
# [doc = " An X or Y axis for the [`Chart`] widget"] # [doc = ""] # [doc = " An axis can have a [title](Axis::title) which will be displayed at the end of the axis. For an"] # [doc = " X axis this is the right, for a Y axis, this is the top."] # [doc = ""] # [doc = " You can also set the bounds and labels on this axis using respectively [`Axis::bounds`] and"] # [doc = " [`Axis::labels`]."] # [doc = ""] # [doc = " See [`Chart::x_axis`] and [`Chart::y_axis`] to set an axis on a chart."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use ratatui::style::{Style, Stylize};"] # [doc = " use ratatui::widgets::Axis;"] # [doc = ""] # [doc = " let axis = Axis::default()"] # [doc = "     .title(\"X Axis\")"] # [doc = "     .style(Style::default().gray())"] # [doc = "     .bounds([0.0, 50.0])"] # [doc = "     .labels([\"0\".bold(), \"25\".into(), \"50\".bold()]);"] # [doc = " ```"] # [derive (Debug , Default , Clone , PartialEq)] pub struct Axis < 'a > { # [doc = " Title displayed next to axis end"] title : Option < Line < 'a > > , # [doc = " Bounds for the axis (all data points outside these limits will not be represented)"] bounds : [f64 ; 2] , # [doc = " A list of labels to put to the left or below the axis"] labels : Vec < Line < 'a > > , # [doc = " The style used to draw the axis itself"] style : Style , # [doc = " The alignment of the labels of the Axis"] labels_alignment : Alignment , }
};
}
