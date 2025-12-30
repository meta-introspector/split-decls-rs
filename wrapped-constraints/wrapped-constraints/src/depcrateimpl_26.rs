// Generated macro for impl_26 (impl)
macro_rules! Depcrateimpl_26 {
() => {
// Module: crate
// Provides: {"impl_26"}
// Dependencies: {}
impl Widget for App { fn render (self , area : Rect , buf : & mut Buffer) { let [tabs , axis , demo] = area . layout (& Layout :: vertical ([Length (3) , Length (3) , Fill (0)])) ; self . render_tabs (tabs , buf) ; Self :: render_axis (axis , buf) ; self . render_demo (demo , buf) ; } }
};
}
