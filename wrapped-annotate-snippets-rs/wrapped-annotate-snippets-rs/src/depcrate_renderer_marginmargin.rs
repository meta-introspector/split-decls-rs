// Generated macro for Margin (struct)
macro_rules! Depcrate_renderer_marginMargin {
() => {
// Module: crate::renderer::margin
// Provides: {"Margin"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , PartialEq)] pub (crate) struct Margin { # [doc = " The available whitespace in the left that can be consumed when centering."] whitespace_left : usize , # [doc = " The column of the beginning of left-most span."] span_left : usize , # [doc = " The column of the end of right-most span."] span_right : usize , # [doc = " The beginning of the line to be displayed."] computed_left : usize , # [doc = " The end of the line to be displayed."] computed_right : usize , # [doc = " The current width of the terminal. 140 by default and in tests."] pub (crate) term_width : usize , # [doc = " The end column of a span label, including the span. Doesn't account for labels not in the"] # [doc = " same line as the span."] label_right : usize , }
};
}
