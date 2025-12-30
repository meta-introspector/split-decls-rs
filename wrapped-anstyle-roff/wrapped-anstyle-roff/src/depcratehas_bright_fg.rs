// Generated macro for has_bright_fg (function)
macro_rules! Depcratehas_bright_fg {
() => {
// Module: crate
// Provides: {"has_bright_fg"}
// Dependencies: {}
fn has_bright_fg (style : & Style) -> bool { style . get_fg_color () . as_ref () . map (is_bright) . unwrap_or (false) }
};
}
