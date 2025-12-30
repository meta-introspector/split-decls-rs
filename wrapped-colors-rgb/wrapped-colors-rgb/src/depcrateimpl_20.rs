// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
# [doc = " Widget impl for `FpsWidget`"] # [doc = ""] # [doc = " This is implemented on a mutable reference so that we can update the frame count and fps"] # [doc = " calculation while rendering."] impl Widget for & mut FpsWidget { fn render (self , area : Rect , buf : & mut Buffer) { self . calculate_fps () ; if let Some (fps) = self . fps { let text = format ! ("{fps:.1} fps") ; Text :: from (text) . render (area , buf) ; } } }
};
}
