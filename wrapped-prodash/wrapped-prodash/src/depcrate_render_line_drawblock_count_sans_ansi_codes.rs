// Generated macro for block_count_sans_ansi_codes (function)
macro_rules! Depcrate_render_line_drawblock_count_sans_ansi_codes {
() => {
// Module: crate::render::line::draw
// Provides: {"block_count_sans_ansi_codes"}
// Dependencies: {}
fn block_count_sans_ansi_codes (strings : & [ANSIString < '_ >]) -> u16 { strings . iter () . map (| s | s . width () as u16) . sum () }
};
}
