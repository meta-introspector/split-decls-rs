// Generated macro for impl_372 (impl)
macro_rules! Depcrate_reflowimpl_372 {
() => {
// Module: crate::reflow
// Provides: {"impl_372"}
// Dependencies: {}
impl < 'a , O , I > LineTruncator < 'a , O , I > where O : Iterator < Item = (I , Alignment) > , I : Iterator < Item = StyledGrapheme < 'a > > , { # [doc = " Create a new `LineTruncator` with the given lines and maximum line width."] pub const fn new (lines : O , max_line_width : u16) -> Self { Self { input_lines : lines , max_line_width , horizontal_offset : 0 , current_line : vec ! [] , } } # [doc = " Set the horizontal offset to skip render."] pub const fn set_horizontal_offset (& mut self , horizontal_offset : u16) { self . horizontal_offset = horizontal_offset ; } }
};
}
