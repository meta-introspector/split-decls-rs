// Generated macro for other_4855 (other)
macro_rules! Depcrate_generatedother_4855 {
() => {
// Module: crate::generated
// Provides: {"other_4855"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Returns an ID for the registry entry that is global to all tasks."] # [doc = ""] # [doc = " The entry ID returned by IORegistryEntryGetRegistryEntryID can be used to identify a registry entry across all tasks. A registry entry may be looked up by its entryID by creating a matching dictionary with IORegistryEntryIDMatching() to be used with the IOKit matching functions. The ID is valid only until the machine reboots."] # [doc = ""] # [doc = " Parameter `entry`: The registry entry handle whose ID to look up."] # [doc = ""] # [doc = " Parameter `entryID`: The resulting ID."] # [doc = ""] # [doc = " Returns: A kern_return_t error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `entry_id` must be a valid pointer."] # [cfg (feature = "libc")] pub fn IORegistryEntryGetRegistryEntryID (entry : io_registry_entry_t , entry_id : * mut u64 ,) -> libc :: kern_return_t ; }
};
}
