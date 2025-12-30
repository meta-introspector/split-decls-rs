// Generated macro for other_4856 (other)
macro_rules! Depcrate_generatedother_4856 {
() => {
// Module: crate::generated
// Provides: {"other_4856"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Create a CF dictionary representation of a registry entry's property table."] # [doc = ""] # [doc = " This function creates an instantaneous snapshot of a registry entry's property table, creating a CFDictionary analogue in the caller's task. Not every object available in the kernel is represented as a CF container; currently OSDictionary, OSArray, OSSet, OSSymbol, OSString, OSData, OSNumber, OSBoolean are created as their CF counterparts."] # [doc = ""] # [doc = " Parameter `entry`: The registry entry handle whose property table to copy."] # [doc = ""] # [doc = " Parameter `properties`: A CFDictionary is created and returned the caller on success. The caller should release with CFRelease."] # [doc = ""] # [doc = " Parameter `allocator`: The CF allocator to use when creating the CF containers."] # [doc = ""] # [doc = " Parameter `options`: No options are currently defined."] # [doc = ""] # [doc = " Returns: A kern_return_t error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `properties` must be a valid pointer."] # [doc = " - `allocator` might not allow `None`."] # [cfg (feature = "libc")] pub fn IORegistryEntryCreateCFProperties (entry : io_registry_entry_t , properties : * mut * mut CFMutableDictionary , allocator : Option < & CFAllocator > , options : IOOptionBits ,) -> libc :: kern_return_t ; }
};
}
