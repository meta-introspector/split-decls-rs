// Generated macro for impl_290 (impl)
macro_rules! Depcrate_sparklineimpl_290 {
() => {
// Module: crate::sparkline
// Provides: {"impl_290"}
// Dependencies: {}
impl Widget for & Sparkline < '_ > { fn render (self , area : Rect , buf : & mut Buffer) { self . block . as_ref () . render (area , buf) ; let inner = self . block . inner_if_some (area) ; self . render_sparkline (inner , buf) ; } }
};
}
