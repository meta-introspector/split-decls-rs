// Generated macro for IOIteratorNext (function)
macro_rules! Depcrate_generatedIOIteratorNext {
() => {
// Module: crate::generated
// Provides: {"IOIteratorNext"}
// Dependencies: {}
# [doc = " Returns the next object in an iteration."] # [doc = ""] # [doc = " This function returns the next object in an iteration, or zero if no more remain or the iterator is invalid."] # [doc = ""] # [doc = " Parameter `iterator`: An IOKit iterator handle."] # [doc = ""] # [doc = " Returns: If the iterator handle is valid, the next element in the iteration is returned, otherwise zero is returned. The element should be released by the caller when it is finished."] # [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IOIteratorNext (iterator : io_iterator_t) -> io_object_t { extern "C-unwind" { fn IOIteratorNext (iterator : io_iterator_t) -> io_object_t ; } unsafe { IOIteratorNext (iterator) } }
};
}
