// Generated macro for HandleProgress (type)
macro_rules! Depcrate_client_async_io_bufread_extHandleProgress {
() => {
// Module: crate::client::async_io::bufread_ext
// Provides: {"HandleProgress"}
// Dependencies: {}
# [doc = " A function `f(is_error, text)` receiving progress or error information."] # [doc = " As it is not a future itself, it must not block. If IO is performed within the function, be sure to spawn"] # [doc = " it onto an executor."] pub type HandleProgress < 'a > = Box < dyn FnMut (bool , & [u8]) -> ProgressAction + 'a > ;
};
}
