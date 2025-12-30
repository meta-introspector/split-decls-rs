// Generated macro for CallbackFunc (struct)
macro_rules! Depcrate_deflate_coreCallbackFunc {
() => {
// Module: crate::deflate::core
// Provides: {"CallbackFunc"}
// Dependencies: {}
# [doc = " Callback function and user used in `compress_to_output`."] pub struct CallbackFunc < 'a > { pub put_buf_func : & 'a mut dyn FnMut (& [u8]) -> bool , }
};
}
