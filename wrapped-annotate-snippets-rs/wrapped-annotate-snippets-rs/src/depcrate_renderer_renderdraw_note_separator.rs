// Generated macro for draw_note_separator (function)
macro_rules! Depcrate_renderer_renderdraw_note_separator {
() => {
// Module: crate::renderer::render
// Provides: {"draw_note_separator"}
// Dependencies: {}
fn draw_note_separator (renderer : & Renderer , buffer : & mut StyledBuffer , line : usize , col : usize , is_cont : bool ,) { let chr = renderer . decor_style . note_separator (is_cont) ; buffer . puts (line , col , chr , ElementStyle :: LineNumber) ; }
};
}
