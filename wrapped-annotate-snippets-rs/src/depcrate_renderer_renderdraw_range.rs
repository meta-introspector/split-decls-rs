// Generated macro for draw_range (function)
macro_rules! Depcrate_renderer_renderdraw_range {
() => {
// Module: crate::renderer::render
// Provides: {"draw_range"}
// Dependencies: {}
fn draw_range (buffer : & mut StyledBuffer , symbol : char , line : usize , col_from : usize , col_to : usize , style : ElementStyle ,) { for col in col_from .. col_to { buffer . putc (line , col , symbol , style) ; } }
};
}
