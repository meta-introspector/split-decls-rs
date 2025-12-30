// Generated macro for other_4862 (other)
macro_rules! Depcrate_generatedother_4862 {
() => {
// Module: crate::generated
// Provides: {"other_4862"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Returns an iterator over an registry entry's child entries in a plane."] # [doc = ""] # [doc = " This method creates an iterator which will return each of a registry entry's child entries in a specified plane."] # [doc = ""] # [doc = " Parameter `entry`: The registry entry whose children to iterate over."] # [doc = ""] # [doc = " Parameter `plane`: The name of an existing registry plane. Plane names are defined in IOKitKeys.h, eg. kIOServicePlane."] # [doc = ""] # [doc = " Parameter `iterator`: The created iterator over the children of the entry, on success. The iterator must be released when the iteration is finished."] # [doc = ""] # [doc = " Returns: A kern_return_t error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `plane` must be a valid pointer."] # [doc = " - `iterator` must be a valid pointer."] # [cfg (feature = "libc")] pub fn IORegistryEntryGetChildIterator (entry : io_registry_entry_t , plane : * mut io_name_t , iterator : * mut io_iterator_t ,) -> libc :: kern_return_t ; }
};
}
