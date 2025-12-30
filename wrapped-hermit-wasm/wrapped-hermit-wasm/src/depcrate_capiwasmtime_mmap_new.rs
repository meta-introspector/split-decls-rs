// Generated macro for wasmtime_mmap_new (function)
macro_rules! Depcrate_capiwasmtime_mmap_new {
() => {
// Module: crate::capi
// Provides: {"wasmtime_mmap_new"}
// Dependencies: {}
# [doc = " Creates a new virtual memory mapping of the `size` specified with"] # [doc = " protection bits specified in `prot_flags`."] # [doc = ""] # [doc = " Memory can be lazily committed."] # [doc = ""] # [doc = " Stores the base pointer of the new mapping in `ret` on success."] # [doc = ""] # [doc = " Returns 0 on success and an error code on failure."] # [doc = ""] # [doc = " Similar to `mmap(0, size, prot_flags, MAP_PRIVATE, 0, -1)` on Linux."] # [unsafe (no_mangle)] pub extern "C" fn wasmtime_mmap_new (size : usize , prot_flags : u32 , ret : & mut * mut u8) -> i32 { unsafe { hermit_abi :: mmap (size , prot_flags , ret) } }
};
}
