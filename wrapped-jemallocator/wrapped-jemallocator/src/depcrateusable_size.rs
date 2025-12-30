// Generated macro for usable_size (function)
macro_rules! Depcrateusable_size {
() => {
// Module: crate
// Provides: {"usable_size"}
// Dependencies: {}
# [doc = " Return the usable size of the allocation pointed to by ptr."] # [doc = ""] # [doc = " The return value may be larger than the size that was requested during allocation."] # [doc = " This function is not a mechanism for in-place `realloc()`;"] # [doc = " rather it is provided solely as a tool for introspection purposes."] # [doc = " Any discrepancy between the requested allocation size"] # [doc = " and the size reported by this function should not be depended on,"] # [doc = " since such behavior is entirely implementation-dependent."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `ptr` must have been allocated by `Jemalloc` and must not have been freed yet."] pub unsafe fn usable_size < T > (ptr : * const T) -> usize { ffi :: malloc_usable_size (ptr as * const c_void) }
};
}
