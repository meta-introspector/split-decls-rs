// Generated macro for impl_342 (impl)
macro_rules! Depcrate_tabsimpl_342 {
() => {
// Module: crate::tabs
// Provides: {"impl_342"}
// Dependencies: {}
impl Widget for & Tabs < '_ > { fn render (self , area : Rect , buf : & mut Buffer) { buf . set_style (area , self . style) ; self . block . as_ref () . render (area , buf) ; let inner = self . block . inner_if_some (area) ; self . render_tabs (inner , buf) ; } }
};
}
