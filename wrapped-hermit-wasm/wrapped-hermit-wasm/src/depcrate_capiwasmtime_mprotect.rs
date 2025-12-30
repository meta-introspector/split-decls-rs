// Generated macro for wasmtime_mprotect (function)
macro_rules! Depcrate_capiwasmtime_mprotect {
() => {
// Module: crate::capi
// Provides: {"wasmtime_mprotect"}
// Dependencies: {}
# [doc = " Configures the protections associated with a region of virtual memory"] # [doc = " starting at `ptr` and going to `size`."] # [doc = ""] # [doc = " Returns 0 on success and an error code on failure."] # [doc = ""] # [doc = " Similar to `mprotect` on Linux."] # [unsafe (no_mangle)] pub extern "C" fn wasmtime_mprotect (ptr : * mut u8 , size : usize , prot_flags : u32) -> i32 { unsafe { hermit_abi :: mprotect (ptr , size , prot_flags) } }
};
}
