// Generated macro for IORegistryGetRootEntry (function)
macro_rules! Depcrate_generatedIORegistryGetRootEntry {
() => {
// Module: crate::generated
// Provides: {"IORegistryGetRootEntry"}
// Dependencies: {}
# [doc = " Return a handle to the registry root."] # [doc = ""] # [doc = " This method provides an accessor to the root of the registry for the machine. The root may be passed to a registry iterator when iterating a plane, and contains properties that describe the available planes, and diagnostic information for IOKit."] # [doc = ""] # [doc = " Parameter `mainPort`: The main port obtained from IOMainPort(). Pass kIOMainPortDefault to look up the default main port."] # [doc = ""] # [doc = " Returns: A handle to the IORegistryEntry root instance, to be released with IOObjectRelease by the caller, or MACH_PORT_NULL on failure."] # [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IORegistryGetRootEntry (main_port : libc :: mach_port_t ,) -> io_registry_entry_t { extern "C-unwind" { fn IORegistryGetRootEntry (main_port : libc :: mach_port_t) -> io_registry_entry_t ; } unsafe { IORegistryGetRootEntry (main_port) } }
};
}
