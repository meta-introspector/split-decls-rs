// Generated macro for draw_multiline_line (function)
macro_rules! Depcrate_renderer_renderdraw_multiline_line {
() => {
// Module: crate::renderer::render
// Provides: {"draw_multiline_line"}
// Dependencies: {}
fn draw_multiline_line (renderer : & Renderer , buffer : & mut StyledBuffer , line : usize , offset : usize , depth : usize , style : ElementStyle ,) { let chr = match (style , renderer . decor_style) { (ElementStyle :: UnderlinePrimary | ElementStyle :: LabelPrimary , DecorStyle :: Ascii) => '|' , (_ , DecorStyle :: Ascii) => '|' , (ElementStyle :: UnderlinePrimary | ElementStyle :: LabelPrimary , DecorStyle :: Unicode) => '┃' , (_ , DecorStyle :: Unicode) => '│' , } ; buffer . putc (line , offset + depth - 1 , chr , style) ; }
};
}
