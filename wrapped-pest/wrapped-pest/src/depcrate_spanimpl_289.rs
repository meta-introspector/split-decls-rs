// Generated macro for impl_289 (impl)
macro_rules! Depcrate_spanimpl_289 {
() => {
// Module: crate::span
// Provides: {"impl_289"}
// Dependencies: {}
impl < 'i > Iterator for LinesSpan < 'i > { type Item = Span < 'i > ; fn next (& mut self) -> Option < Self :: Item > { if self . pos > self . span . end { return None ; } let pos = position :: Position :: new (self . span . input , self . pos) ? ; if pos . at_end () { return None ; } let line_start = pos . find_line_start () ; self . pos = pos . find_line_end () ; Span :: new (self . span . input , line_start , self . pos) } }
};
}
