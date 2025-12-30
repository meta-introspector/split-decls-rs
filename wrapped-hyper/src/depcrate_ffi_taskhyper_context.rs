// Generated macro for hyper_context (struct)
macro_rules! Depcrate_ffi_taskhyper_context {
() => {
// Module: crate::ffi::task
// Provides: {"hyper_context"}
// Dependencies: {}
# [doc = " An async context for a task that contains the related waker."] # [doc = ""] # [doc = " This is provided to `hyper_io`'s read and write callbacks. Currently"] # [doc = " its only purpose is to provide access to the waker. See `hyper_waker`."] # [doc = ""] # [doc = " Corresponding Rust type: <https://doc.rust-lang.org/std/task/struct.Context.html>"] pub struct hyper_context < 'a > (Context < 'a >) ;
};
}
