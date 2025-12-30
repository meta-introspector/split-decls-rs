// Generated macro for render_header (function)
macro_rules! Depcraterender_header {
() => {
// Module: crate
// Provides: {"render_header"}
// Dependencies: {}
fn render_header (area : Rect , buf : & mut Buffer) { Paragraph :: new ("Ratatui Gauge Example") . bold () . alignment (Alignment :: Center) . fg (CUSTOM_LABEL_COLOR) . render (area , buf) ; }
};
}
