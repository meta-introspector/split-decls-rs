// Generated macro for other_4879 (other)
macro_rules! Depcrate_generatedother_4879 {
() => {
// Module: crate::generated
// Provides: {"other_4879"}
// Dependencies: {}
extern "C-unwind" { # [doc = " # Safety"] # [doc = ""] # [doc = " - `buffer` must be a valid pointer."] # [doc = " - `size` must be a valid pointer."] # [cfg (feature = "libc")] pub fn IOCatalogueGetData (main_port : libc :: mach_port_t , flag : u32 , buffer : * mut * mut c_char , size : * mut u32 ,) -> libc :: kern_return_t ; }
};
}
