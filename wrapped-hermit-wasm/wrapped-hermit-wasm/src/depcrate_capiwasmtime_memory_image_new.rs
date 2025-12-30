// Generated macro for wasmtime_memory_image_new (function)
macro_rules! Depcrate_capiwasmtime_memory_image_new {
() => {
// Module: crate::capi
// Provides: {"wasmtime_memory_image_new"}
// Dependencies: {}
# [doc = " Attempts to create a new in-memory image of the `ptr`/`len` combo which"] # [doc = " can be mapped to virtual addresses in the future."] # [doc = ""] # [doc = " On success the returned `wasmtime_memory_image` pointer is stored into `ret`."] # [doc = " This value stored can be `NULL` to indicate that an image cannot be"] # [doc = " created but no failure occurred. The structure otherwise will later be"] # [doc = " deallocated with `wasmtime_memory_image_free` and"] # [doc = " `wasmtime_memory_image_map_at` will be used to map the image into new"] # [doc = " regions of the address space."] # [doc = ""] # [doc = " The `ptr` and `len` arguments are only valid for this function call, if"] # [doc = " the image needs to refer to them in the future then it must make a copy."] # [doc = ""] # [doc = " Both `ptr` and `len` are guaranteed to be page-aligned."] # [doc = ""] # [doc = " Returns 0 on success and an error code on failure. Note that storing"] # [doc = " `NULL` into `ret` is not considered a failure, and failure is used to"] # [doc = " indicate that something fatal has happened and Wasmtime will propagate"] # [doc = " the error upwards."] # [unsafe (no_mangle)] pub extern "C" fn wasmtime_memory_image_new (_ptr : * const u8 , _len : usize , ret : & mut * mut wasmtime_memory_image ,) -> i32 { * ret = std :: ptr :: null_mut () ; 0 }
};
}
