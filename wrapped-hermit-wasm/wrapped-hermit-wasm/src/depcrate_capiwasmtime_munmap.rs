// Generated macro for wasmtime_munmap (function)
macro_rules! Depcrate_capiwasmtime_munmap {
() => {
// Module: crate::capi
// Provides: {"wasmtime_munmap"}
// Dependencies: {}
# [doc = " Unmaps memory at the specified `ptr` for `size` bytes."] # [doc = ""] # [doc = " The memory should be discarded and decommitted and should generate a"] # [doc = " segfault if accessed after this function call."] # [doc = ""] # [doc = " Returns 0 on success and an error code on failure."] # [doc = ""] # [doc = " Similar to `munmap` on Linux."] # [unsafe (no_mangle)] pub extern "C" fn wasmtime_munmap (ptr : * mut u8 , size : usize) -> i32 { unsafe { hermit_abi :: munmap (ptr , size) } }
};
}
