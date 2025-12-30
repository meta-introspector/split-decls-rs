// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl Widget for App { fn render (self , area : Rect , buf : & mut Buffer) { let layout = Layout :: vertical ([Length (3) , Length (1) , Fill (0)]) ; let [tabs , axis , demo] = area . layout (& layout) ; self . tabs () . render (tabs , buf) ; let scroll_needed = self . render_demo (demo , buf) ; let axis_width = if scroll_needed { axis . width . saturating_sub (1) } else { axis . width } ; Self :: axis (axis_width , self . spacing) . render (axis , buf) ; } }
};
}
