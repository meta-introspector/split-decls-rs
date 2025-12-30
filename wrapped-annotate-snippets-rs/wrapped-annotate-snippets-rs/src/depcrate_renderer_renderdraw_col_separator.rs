// Generated macro for draw_col_separator (function)
macro_rules! Depcrate_renderer_renderdraw_col_separator {
() => {
// Module: crate::renderer::render
// Provides: {"draw_col_separator"}
// Dependencies: {}
fn draw_col_separator (renderer : & Renderer , buffer : & mut StyledBuffer , line : usize , col : usize) { let chr = renderer . decor_style . col_separator () ; buffer . puts (line , col , & format ! ("{chr} ") , ElementStyle :: LineNumber) ; }
};
}
