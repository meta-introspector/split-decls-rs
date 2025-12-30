// Generated macro for render_footer (function)
macro_rules! Depcraterender_footer {
() => {
// Module: crate
// Provides: {"render_footer"}
// Dependencies: {}
fn render_footer (area : Rect , buf : & mut Buffer) { Paragraph :: new ("Press ENTER to start") . alignment (Alignment :: Center) . fg (CUSTOM_LABEL_COLOR) . bold () . render (area , buf) ; }
};
}
