// Generated macro for IORegistryIteratorExitEntry (function)
macro_rules! Depcrate_generatedIORegistryIteratorExitEntry {
() => {
// Module: crate::generated
// Provides: {"IORegistryIteratorExitEntry"}
// Dependencies: {}
# [doc = " Exits a level of recursion, restoring the current entry."] # [doc = ""] # [doc = " This method undoes an IORegistryIteratorEnterEntry, restoring the current entry. If there are no more levels of recursion to exit false is returned, otherwise true is returned."] # [doc = ""] # [doc = " Returns: kIOReturnSuccess if a level of recursion was undone, kIOReturnNoDevice if no recursive levels are left in the iteration."] # [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IORegistryIteratorExitEntry (iterator : io_iterator_t ,) -> libc :: kern_return_t { extern "C-unwind" { fn IORegistryIteratorExitEntry (iterator : io_iterator_t) -> libc :: kern_return_t ; } unsafe { IORegistryIteratorExitEntry (iterator) } }
};
}
