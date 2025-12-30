// Generated macro for draw_col_separator_no_space_with_style (function)
macro_rules! Depcrate_renderer_renderdraw_col_separator_no_space_with_style {
() => {
// Module: crate::renderer::render
// Provides: {"draw_col_separator_no_space_with_style"}
// Dependencies: {}
fn draw_col_separator_no_space_with_style (buffer : & mut StyledBuffer , chr : char , line : usize , col : usize , style : ElementStyle ,) { buffer . putc (line , col , chr , style) ; }
};
}
