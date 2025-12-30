// Generated macro for macro_2249 (macro)
macro_rules! Depcrate_io_copy_buf_abortablemacro_2249 {
() => {
// Module: crate::io::copy_buf_abortable
// Provides: {"macro_2249"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`copy_buf_abortable()`] function."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct CopyBufAbortable <'a , R , W : ? Sized > { # [pin] reader : R , writer : &'a mut W , amt : u64 , inner : Arc < AbortInner > } }
};
}
