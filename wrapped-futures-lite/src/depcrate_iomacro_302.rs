// Generated macro for macro_302 (macro)
macro_rules! Depcrate_iomacro_302 {
() => {
// Module: crate::io
// Provides: {"macro_302"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`AsyncBufReadExt::lines()`] method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Lines < R > { # [pin] reader : R , buf : String , bytes : Vec < u8 >, read : usize , } }
};
}
