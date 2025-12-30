// Generated macro for draw_col_separator_no_space (function)
macro_rules! Depcrate_renderer_renderdraw_col_separator_no_space {
() => {
// Module: crate::renderer::render
// Provides: {"draw_col_separator_no_space"}
// Dependencies: {}
fn draw_col_separator_no_space (renderer : & Renderer , buffer : & mut StyledBuffer , line : usize , col : usize ,) { let chr = renderer . decor_style . col_separator () ; draw_col_separator_no_space_with_style (buffer , chr , line , col , ElementStyle :: LineNumber) ; }
};
}
