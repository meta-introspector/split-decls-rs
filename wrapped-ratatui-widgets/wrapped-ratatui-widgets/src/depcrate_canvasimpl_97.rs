// Generated macro for impl_97 (impl)
macro_rules! Depcrate_canvasimpl_97 {
() => {
// Module: crate::canvas
// Provides: {"impl_97"}
// Dependencies: {}
impl CharGrid { # [doc = " Create a new `CharGrid` with the given width and height measured in terminal columns and"] # [doc = " rows respectively."] fn new (width : u16 , height : u16 , cell_char : char) -> Self { let length = usize :: from (width) * usize :: from (height) ; Self { width , height , cells : vec ! [' ' ; length] , colors : vec ! [Color :: Reset ; length] , cell_char , } } }
};
}
