// Generated macro for other_4846 (other)
macro_rules! Depcrate_generatedother_4846 {
() => {
// Module: crate::generated
// Provides: {"other_4846"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Create an iterator rooted at the registry root."] # [doc = ""] # [doc = " This method creates an IORegistryIterator in the kernel that is set up with options to iterate children of the registry root entry, and to recurse automatically into entries as they are returned, or only when instructed with calls to IORegistryIteratorEnterEntry. The iterator object keeps track of entries that have been recursed into previously to avoid loops."] # [doc = ""] # [doc = " Parameter `mainPort`: The main port obtained from IOMainPort(). Pass kIOMainPortDefault to look up the default main port."] # [doc = ""] # [doc = " Parameter `plane`: The name of an existing registry plane. Plane names are defined in IOKitKeys.h, eg. kIOServicePlane."] # [doc = ""] # [doc = " Parameter `options`: kIORegistryIterateRecursively may be set to recurse automatically into each entry as it is returned from IOIteratorNext calls on the registry iterator."] # [doc = ""] # [doc = " Parameter `iterator`: A created iterator handle, to be released by the caller when it has finished with it."] # [doc = ""] # [doc = " Returns: A kern_return_t error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `plane` must be a valid pointer."] # [doc = " - `iterator` must be a valid pointer."] # [cfg (feature = "libc")] pub fn IORegistryCreateIterator (main_port : libc :: mach_port_t , plane : * mut io_name_t , options : IOOptionBits , iterator : * mut io_iterator_t ,) -> libc :: kern_return_t ; }
};
}
