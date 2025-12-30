// Generated macro for impl_853 (impl)
macro_rules! Depcrate_output_tableimpl_853 {
() => {
// Module: crate::output::table
// Provides: {"impl_853"}
// Dependencies: {}
impl TableWidths { pub fn zero (count : usize) -> Self { Self (vec ! [0 ; count]) } pub fn add_widths (& mut self , row : & Row) { for (old_width , cell) in self . 0 . iter_mut () . zip (row . cells . iter ()) { * old_width = max (* old_width , * cell . width) ; } } pub fn total (& self) -> usize { self . 0 . len () + self . 0 . iter () . sum :: < usize > () } }
};
}
