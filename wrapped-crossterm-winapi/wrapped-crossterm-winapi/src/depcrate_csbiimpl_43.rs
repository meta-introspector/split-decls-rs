// Generated macro for impl_43 (impl)
macro_rules! Depcrate_csbiimpl_43 {
() => {
// Module: crate::csbi
// Provides: {"impl_43"}
// Dependencies: {}
impl fmt :: Debug for ScreenBufferInfo { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("ScreenBufferInfo") . field ("dwSize" , & self . buffer_size ()) . field ("dwCursorPosition" , & self . cursor_pos ()) . field ("wAttributes" , & self . attributes ()) . field ("srWindow" , & self . terminal_window ()) . field ("dwMaximumWindowSize" , & Size :: from (self . 0 . dwMaximumWindowSize) ,) . finish () } }
};
}
