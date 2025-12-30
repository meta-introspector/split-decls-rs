// Generated macro for wasmtime_mmap_remap (function)
macro_rules! Depcrate_capiwasmtime_mmap_remap {
() => {
// Module: crate::capi
// Provides: {"wasmtime_mmap_remap"}
// Dependencies: {}
# [doc = " Remaps the virtual memory starting at `addr` going for `size` bytes to"] # [doc = " the protections specified with a new blank mapping."] # [doc = ""] # [doc = " This will unmap any prior mappings and decommit them. New mappings for"] # [doc = " anonymous memory are used to replace these mappings and the new area"] # [doc = " should have the protection specified by `prot_flags`."] # [doc = ""] # [doc = " Returns 0 on success and an error code on failure."] # [doc = ""] # [doc = " Similar to `mmap(addr, size, prot_flags, MAP_PRIVATE | MAP_FIXED, 0, -1)` on Linux."] # [unsafe (no_mangle)] pub extern "C" fn wasmtime_mmap_remap (_addr : * mut u8 , _size : usize , _prot_flags : WasmProt) -> i32 { error ! ("Currently. HermitOS doesn't support wasmtime_mmap_remap!") ; - 1 }
};
}
