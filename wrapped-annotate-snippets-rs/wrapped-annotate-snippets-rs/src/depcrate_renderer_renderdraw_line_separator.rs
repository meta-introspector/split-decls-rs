// Generated macro for draw_line_separator (function)
macro_rules! Depcrate_renderer_renderdraw_line_separator {
() => {
// Module: crate::renderer::render
// Provides: {"draw_line_separator"}
// Dependencies: {}
fn draw_line_separator (renderer : & Renderer , buffer : & mut StyledBuffer , line : usize , col : usize) { let (column , dots) = match renderer . decor_style { DecorStyle :: Ascii => (0 , "...") , DecorStyle :: Unicode => (col - 2 , "‡") , } ; buffer . puts (line , column , dots , ElementStyle :: LineNumber) ; }
};
}
