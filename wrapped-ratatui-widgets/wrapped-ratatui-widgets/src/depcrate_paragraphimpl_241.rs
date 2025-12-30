// Generated macro for impl_241 (impl)
macro_rules! Depcrate_paragraphimpl_241 {
() => {
// Module: crate::paragraph
// Provides: {"impl_241"}
// Dependencies: {}
impl Paragraph < '_ > { fn render_paragraph (& self , text_area : Rect , buf : & mut Buffer) { if text_area . is_empty () { return ; } buf . set_style (text_area , self . style) ; let styled = self . text . iter () . map (| line | { let graphemes = line . styled_graphemes (self . text . style) ; let alignment = line . alignment . unwrap_or (self . alignment) ; (graphemes , alignment) }) ; if let Some (Wrap { trim }) = self . wrap { let mut line_composer = WordWrapper :: new (styled , text_area . width , trim) ; for _ in 0 .. self . scroll . y { if line_composer . next_line () . is_none () { return ; } } render_lines (line_composer , text_area , buf) ; } else { let lines = styled . skip (self . scroll . y as usize) ; let mut line_composer = LineTruncator :: new (lines , text_area . width) ; line_composer . set_horizontal_offset (self . scroll . x) ; render_lines (line_composer , text_area , buf) ; } } }
};
}
