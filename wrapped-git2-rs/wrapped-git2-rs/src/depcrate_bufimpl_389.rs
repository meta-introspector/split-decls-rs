// Generated macro for impl_389 (impl)
macro_rules! Depcrate_bufimpl_389 {
() => {
// Module: crate::buf
// Provides: {"impl_389"}
// Dependencies: {}
impl Buf { # [doc = " Creates a new empty buffer."] pub fn new () -> Buf { crate :: init () ; unsafe { Binding :: from_raw (& mut raw :: git_buf { ptr : ptr :: null_mut () , size : 0 , reserved : 0 , } as * mut _) } } # [doc = " Attempt to view this buffer as a string slice."] # [doc = ""] # [doc = " Returns `None` if the buffer is not valid utf-8."] pub fn as_str (& self) -> Option < & str > { str :: from_utf8 (& * * self) . ok () } }
};
}
