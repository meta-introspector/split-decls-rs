// Generated macro for impl_44 (impl)
macro_rules! Depcrate_blockimpl_44 {
() => {
// Module: crate::block
// Provides: {"impl_44"}
// Dependencies: {}
impl Widget for & Block < '_ > { fn render (self , area : Rect , buf : & mut Buffer) { let area = area . intersection (buf . area) ; if area . is_empty () { return ; } buf . set_style (area , self . style) ; self . render_borders (area , buf) ; self . render_titles (area , buf) ; } }
};
}
