// Generated macro for impl_74 (impl)
macro_rules! Depcrate_astimpl_74 {
() => {
// Module: crate::ast
// Provides: {"impl_74"}
// Dependencies: {}
impl Span { # [doc = " Create a new span with the given positions."] pub fn new (start : Position , end : Position) -> Span { Span { start , end } } # [doc = " Create a new span using the given position as the start and end."] pub fn splat (pos : Position) -> Span { Span :: new (pos , pos) } # [doc = " Create a new span by replacing the starting the position with the one"] # [doc = " given."] pub fn with_start (self , pos : Position) -> Span { Span { start : pos , .. self } } # [doc = " Create a new span by replacing the ending the position with the one"] # [doc = " given."] pub fn with_end (self , pos : Position) -> Span { Span { end : pos , .. self } } # [doc = " Returns true if and only if this span occurs on a single line."] pub fn is_one_line (& self) -> bool { self . start . line == self . end . line } # [doc = " Returns true if and only if this span is empty. That is, it points to"] # [doc = " a single position in the concrete syntax of a regular expression."] pub fn is_empty (& self) -> bool { self . start . offset == self . end . offset } }
};
}
