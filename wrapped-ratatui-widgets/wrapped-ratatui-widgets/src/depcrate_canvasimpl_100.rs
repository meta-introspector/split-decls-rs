// Generated macro for impl_100 (impl)
macro_rules! Depcrate_canvasimpl_100 {
() => {
// Module: crate::canvas
// Provides: {"impl_100"}
// Dependencies: {}
impl HalfBlockGrid { # [doc = " Create a new `HalfBlockGrid` with the given width and height measured in terminal columns"] # [doc = " and rows respectively."] fn new (width : u16 , height : u16) -> Self { Self { width , height , pixels : vec ! [vec ! [Color :: Reset ; width as usize] ; (height as usize) * 2] , } } }
};
}
