// Generated macro for impl_148 (impl)
macro_rules! Depcrate_clearimpl_148 {
() => {
// Module: crate::clear
// Provides: {"impl_148"}
// Dependencies: {}
impl Widget for & Clear { fn render (self , area : Rect , buf : & mut Buffer) { for x in area . left () .. area . right () { for y in area . top () .. area . bottom () { buf [(x , y)] . reset () ; } } } }
};
}
