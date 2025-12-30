// Generated macro for macro_2331 (macro)
macro_rules! Depcrate_io_linesmacro_2331 {
() => {
// Module: crate::io::lines
// Provides: {"macro_2331"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`lines`](super::AsyncBufReadExt::lines) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Lines < R > { # [pin] reader : R , buf : String , bytes : Vec < u8 >, read : usize , } }
};
}
