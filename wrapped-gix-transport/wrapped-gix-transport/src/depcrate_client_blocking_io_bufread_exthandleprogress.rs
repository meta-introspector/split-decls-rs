// Generated macro for HandleProgress (type)
macro_rules! Depcrate_client_blocking_io_bufread_extHandleProgress {
() => {
// Module: crate::client::blocking_io::bufread_ext
// Provides: {"HandleProgress"}
// Dependencies: {}
# [doc = " A function `f(is_error, text)` receiving progress or error information."] pub type HandleProgress < 'a > = Box < dyn FnMut (bool , & [u8]) -> ProgressAction + 'a > ;
};
}
