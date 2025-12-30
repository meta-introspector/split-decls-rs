// Generated macro for Gauge (struct)
macro_rules! Depcrate_gaugeGauge {
() => {
// Module: crate::gauge
// Provides: {"Gauge"}
// Dependencies: {}
# [doc = " A widget to display a progress bar."] # [doc = ""] # [doc = " A `Gauge` renders a bar filled according to the value given to [`Gauge::percent`] or"] # [doc = " [`Gauge::ratio`]. The bar width and height are defined by the [`Rect`] it is"] # [doc = " [rendered](Widget::render) in."] # [doc = ""] # [doc = " The associated label is always centered horizontally and vertically. If not set with"] # [doc = " [`Gauge::label`], the label is the percentage of the bar filled."] # [doc = ""] # [doc = " You might want to have a higher precision bar using [`Gauge::use_unicode`]."] # [doc = ""] # [doc = " This can be useful to indicate the progression of a task, like a download."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use ratatui::style::{Style, Stylize};"] # [doc = " use ratatui::widgets::{Block, Gauge};"] # [doc = ""] # [doc = " Gauge::default()"] # [doc = "     .block(Block::bordered().title(\"Progress\"))"] # [doc = "     .gauge_style(Style::new().white().on_black().italic())"] # [doc = "     .percent(20);"] # [doc = " ```"] # [doc = ""] # [doc = " # See also"] # [doc = ""] # [doc = " - [`LineGauge`] for a thin progress bar"] # [expect (clippy :: struct_field_names)] # [derive (Debug , Default , Clone , PartialEq)] pub struct Gauge < 'a > { block : Option < Block < 'a > > , ratio : f64 , label : Option < Span < 'a > > , use_unicode : bool , style : Style , gauge_style : Style , }
};
}
