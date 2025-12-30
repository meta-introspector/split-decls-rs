// Generated macro for other_4843 (other)
macro_rules! Depcrate_generatedother_4843 {
() => {
// Module: crate::generated
// Provides: {"other_4843"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Looks up a registry entry by path."] # [doc = ""] # [doc = " This function parses paths to lookup registry entries. The path should begin with '"] # [doc = " <plane"] # [doc = " name>:' If there are characters remaining unparsed after an entry has been looked up, this is considered an invalid lookup. Paths are further documented in IORegistryEntry.h"] # [doc = ""] # [doc = " Parameter `mainPort`: The main port obtained from IOMainPort(). Pass kIOMainPortDefault to look up the default main port."] # [doc = ""] # [doc = " Parameter `path`: A CFString path."] # [doc = ""] # [doc = " Returns: A handle to the IORegistryEntry which was found with the path, to be released with IOObjectRelease by the caller, or MACH_PORT_NULL on failure."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `path` might not allow `None`."] # [cfg (feature = "libc")] pub fn IORegistryEntryCopyFromPath (main_port : libc :: mach_port_t , path : Option < & CFString > ,) -> io_registry_entry_t ; }
};
}
