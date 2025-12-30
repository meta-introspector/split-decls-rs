// Generated macro for draw_col_separator_start (function)
macro_rules! Depcrate_renderer_renderdraw_col_separator_start {
() => {
// Module: crate::renderer::render
// Provides: {"draw_col_separator_start"}
// Dependencies: {}
fn draw_col_separator_start (renderer : & Renderer , buffer : & mut StyledBuffer , line : usize , col : usize ,) { match renderer . decor_style { DecorStyle :: Ascii => { draw_col_separator_no_space_with_style (buffer , '|' , line , col , ElementStyle :: LineNumber ,) ; } DecorStyle :: Unicode => { draw_col_separator_no_space_with_style (buffer , '╭' , line , col , ElementStyle :: LineNumber ,) ; draw_col_separator_no_space_with_style (buffer , '╴' , line , col + 1 , ElementStyle :: LineNumber ,) ; } } }
};
}
