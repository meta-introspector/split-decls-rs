// Generated macro for get_line_offset (function)
macro_rules! Depcrate_paragraphget_line_offset {
() => {
// Module: crate::paragraph
// Provides: {"get_line_offset"}
// Dependencies: {}
const fn get_line_offset (line_width : u16 , text_area_width : u16 , alignment : Alignment) -> u16 { match alignment { Alignment :: Center => (text_area_width / 2) . saturating_sub (line_width / 2) , Alignment :: Right => text_area_width . saturating_sub (line_width) , Alignment :: Left => 0 , } }
};
}
