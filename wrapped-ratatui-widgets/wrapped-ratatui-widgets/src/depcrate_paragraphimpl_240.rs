// Generated macro for impl_240 (impl)
macro_rules! Depcrate_paragraphimpl_240 {
() => {
// Module: crate::paragraph
// Provides: {"impl_240"}
// Dependencies: {}
impl Widget for & Paragraph < '_ > { fn render (self , area : Rect , buf : & mut Buffer) { let area = area . intersection (buf . area) ; buf . set_style (area , self . style) ; self . block . as_ref () . render (area , buf) ; let inner = self . block . inner_if_some (area) ; self . render_paragraph (inner , buf) ; } }
};
}
