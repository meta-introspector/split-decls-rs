// Generated macro for max_scroll_offset (function)
macro_rules! Depcratemax_scroll_offset {
() => {
// Module: crate
// Provides: {"max_scroll_offset"}
// Dependencies: {}
fn max_scroll_offset () -> u16 { example_height () - EXAMPLE_DATA . last () . map_or (0 , | (desc , _) | get_description_height (desc) + 4) }
};
}
