// Generated macro for impl_26 (impl)
macro_rules! Depcrateimpl_26 {
() => {
// Module: crate
// Provides: {"impl_26"}
// Dependencies: {}
impl Widget for SpacerBlock { fn render (self , area : Rect , buf : & mut Buffer) { match area . height { 1 => () , 2 => Self :: render_2px (area , buf) , 3 => Self :: render_3px (area , buf) , _ => Self :: render_4px (area , buf) , } } }
};
}
