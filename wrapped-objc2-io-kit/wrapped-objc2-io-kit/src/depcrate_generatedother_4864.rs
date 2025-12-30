// Generated macro for other_4864 (other)
macro_rules! Depcrate_generatedother_4864 {
() => {
// Module: crate::generated
// Provides: {"other_4864"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Returns an iterator over an registry entry's parent entries in a plane."] # [doc = ""] # [doc = " This method creates an iterator which will return each of a registry entry's parent entries in a specified plane."] # [doc = ""] # [doc = " Parameter `entry`: The registry entry whose parents to iterate over."] # [doc = ""] # [doc = " Parameter `plane`: The name of an existing registry plane. Plane names are defined in IOKitKeys.h, eg. kIOServicePlane."] # [doc = ""] # [doc = " Parameter `iterator`: The created iterator over the parents of the entry, on success. The iterator must be released when the iteration is finished."] # [doc = ""] # [doc = " Returns: A kern_return_t error."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `plane` must be a valid pointer."] # [doc = " - `iterator` must be a valid pointer."] # [cfg (feature = "libc")] pub fn IORegistryEntryGetParentIterator (entry : io_registry_entry_t , plane : * mut io_name_t , iterator : * mut io_iterator_t ,) -> libc :: kern_return_t ; }
};
}
