// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl Widget for ConstraintBlock { fn render (self , area : Rect , buf : & mut Buffer) { match area . height { 1 => self . render_1px (area , buf) , 2 => self . render_2px (area , buf) , _ => self . render_4px (area , buf) , } } }
};
}
