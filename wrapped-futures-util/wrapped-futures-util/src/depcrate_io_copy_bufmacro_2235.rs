// Generated macro for macro_2235 (macro)
macro_rules! Depcrate_io_copy_bufmacro_2235 {
() => {
// Module: crate::io::copy_buf
// Provides: {"macro_2235"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`copy_buf()`] function."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct CopyBuf <'a , R , W : ? Sized > { # [pin] reader : R , writer : &'a mut W , amt : u64 , } }
};
}
