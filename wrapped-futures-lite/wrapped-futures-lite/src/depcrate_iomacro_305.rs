// Generated macro for macro_305 (macro)
macro_rules! Depcrate_iomacro_305 {
() => {
// Module: crate::io
// Provides: {"macro_305"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`AsyncBufReadExt::split()`] method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Split < R > { # [pin] reader : R , buf : Vec < u8 >, read : usize , delim : u8 , } }
};
}
