// Generated macro for other_4850 (other)
macro_rules! Depcrate_generatedother_4850 {
() => {
// Module: crate::generated
// Provides: {"other_4850"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Returns a C-string name assigned to a registry entry."] # [doc = ""] # [doc = " Registry entries can be named in a particular plane, or globally. This function returns the entry's global name. The global name defaults to the entry's meta class name if it has not been named."] # [doc = ""] # [doc = " Parameter `entry`: The registry entry handle whose name to look up."] # [doc = ""] # [doc = " Parameter `name`: The caller's buffer to receive the name."] # [doc = ""] # [doc = " Returns: A kern_return_t error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `name` must be a valid pointer."] # [cfg (feature = "libc")] pub fn IORegistryEntryGetName (entry : io_registry_entry_t , name : * mut io_name_t ,) -> libc :: kern_return_t ; }
};
}
