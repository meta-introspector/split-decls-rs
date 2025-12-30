// Generated macro for other_4859 (other)
macro_rules! Depcrate_generatedother_4859 {
() => {
// Module: crate::generated
// Provides: {"other_4859"}
// Dependencies: {}
extern "C-unwind" { # [doc = " # Safety"] # [doc = ""] # [doc = " - `property_name` must be a valid pointer."] # [doc = " - `buffer` must be a valid pointer."] # [doc = " - `size` must be a valid pointer."] # [cfg (feature = "libc")] pub fn IORegistryEntryGetProperty (entry : io_registry_entry_t , property_name : * mut io_name_t , buffer : * mut io_struct_inband_t , size : * mut u32 ,) -> libc :: kern_return_t ; }
};
}
