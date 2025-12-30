// Generated macro for IOIteratorIsValid (function)
macro_rules! Depcrate_generatedIOIteratorIsValid {
() => {
// Module: crate::generated
// Provides: {"IOIteratorIsValid"}
// Dependencies: {}
# [doc = " Checks an iterator is still valid."] # [doc = ""] # [doc = " Some iterators will be made invalid if changes are made to the structure they are iterating over. This function checks the iterator is still valid and should be called when IOIteratorNext returns zero. An invalid iterator can be reset and the iteration restarted."] # [doc = ""] # [doc = " Parameter `iterator`: An IOKit iterator handle."] # [doc = ""] # [doc = " Returns: True if the iterator handle is valid, otherwise false is returned."] # [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IOIteratorIsValid (iterator : io_iterator_t) -> bool { extern "C-unwind" { fn IOIteratorIsValid (iterator : io_iterator_t) -> libc :: boolean_t ; } let ret = unsafe { IOIteratorIsValid (iterator) } ; ret != 0 }
};
}
