// Generated macro for impl_94 (impl)
macro_rules! Depcrate_canvasimpl_94 {
() => {
// Module: crate::canvas
// Provides: {"impl_94"}
// Dependencies: {}
impl BrailleGrid { # [doc = " Create a new `BrailleGrid` with the given width and height measured in terminal columns and"] # [doc = " rows respectively."] fn new (width : u16 , height : u16) -> Self { let length = usize :: from (width) * usize :: from (height) ; Self { width , height , utf16_code_points : vec ! [symbols :: braille :: BLANK ; length] , colors : vec ! [Color :: Reset ; length] , } } }
};
}
