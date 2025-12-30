// Generated macro for wasmtime_memory_image_map_at (function)
macro_rules! Depcrate_capiwasmtime_memory_image_map_at {
() => {
// Module: crate::capi
// Provides: {"wasmtime_memory_image_map_at"}
// Dependencies: {}
# [doc = " Maps the `image` provided to the virtual address at `addr` and `len`."] # [doc = ""] # [doc = " This semantically should make it such that `addr` and `len` looks the"] # [doc = " same as the contents of what the memory image was first created with."] # [doc = " The mappings of `addr` should be private and changes do not reflect back"] # [doc = " to `wasmtime_memory_image`."] # [doc = ""] # [doc = " In effect this is to create a copy-on-write mapping at `addr`/`len`"] # [doc = " pointing back to the memory used by the image originally."] # [doc = ""] # [doc = " Note that the memory region will be unmapped with `wasmtime_munmap` in"] # [doc = " the future."] # [doc = ""] # [doc = " Aborts the process on failure."] # [unsafe (no_mangle)] pub extern "C" fn wasmtime_memory_image_map_at (_image : * mut wasmtime_memory_image , _addr : * mut u8 , _len : usize ,) -> i32 { error ! ("Currently. HermitOS doesn't support wasmtime_memory_image_map_at!") ; hermit_abi :: ENOSYS }
};
}
