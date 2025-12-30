// Generated macro for IORegistryIteratorEnterEntry (function)
macro_rules! Depcrate_generatedIORegistryIteratorEnterEntry {
() => {
// Module: crate::generated
// Provides: {"IORegistryIteratorEnterEntry"}
// Dependencies: {}
# [doc = " Recurse into the current entry in the registry iteration."] # [doc = ""] # [doc = " This method makes the current entry, ie. the last entry returned by IOIteratorNext, the root in a new level of recursion."] # [doc = ""] # [doc = " Returns: A kern_return_t error code."] # [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IORegistryIteratorEnterEntry (iterator : io_iterator_t ,) -> libc :: kern_return_t { extern "C-unwind" { fn IORegistryIteratorEnterEntry (iterator : io_iterator_t) -> libc :: kern_return_t ; } unsafe { IORegistryIteratorEnterEntry (iterator) } }
};
}
