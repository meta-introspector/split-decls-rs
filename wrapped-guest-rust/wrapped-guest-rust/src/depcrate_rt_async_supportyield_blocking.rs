// Generated macro for yield_blocking (function)
macro_rules! Depcrate_rt_async_supportyield_blocking {
() => {
// Module: crate::rt::async_support
// Provides: {"yield_blocking"}
// Dependencies: {}
# [doc = " Call the `yield` canonical built-in function."] # [doc = ""] # [doc = " This yields control to the host temporarily, allowing other tasks to make"] # [doc = " progress. It's a good idea to call this inside a busy loop which does not"] # [doc = " otherwise ever yield control the host."] # [doc = ""] # [doc = " Note that this function is a blocking function, not an `async` function."] # [doc = " That means that this is not an async yield which allows other tasks in this"] # [doc = " component to progress, but instead this will block the current function"] # [doc = " until the host gets back around to returning from this yield. Asynchronous"] # [doc = " functions should probably use [`yield_async`] instead."] # [doc = ""] # [doc = " # Return Value"] # [doc = ""] # [doc = " This function returns a `bool` which indicates whether execution should"] # [doc = " continue after this yield point. A return value of `true` means that the"] # [doc = " task was not cancelled and execution should continue. A return value of"] # [doc = " `false`, however, means that the task was cancelled while it was suspended"] # [doc = " at this yield point. The caller should return back and exit from the task"] # [doc = " ASAP in this situation."] pub fn yield_blocking () -> bool { # [cfg (not (target_arch = "wasm32"))] unsafe fn yield_ () -> bool { unreachable ! () ; } # [cfg (target_arch = "wasm32")] # [link (wasm_import_module = "$root")] extern "C" { # [link_name = "[thread-yield]"] fn yield_ () -> bool ; } unsafe { ! yield_ () } }
};
}
