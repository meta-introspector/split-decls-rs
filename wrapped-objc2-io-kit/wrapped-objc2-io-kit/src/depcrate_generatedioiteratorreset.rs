// Generated macro for IOIteratorReset (function)
macro_rules! Depcrate_generatedIOIteratorReset {
() => {
// Module: crate::generated
// Provides: {"IOIteratorReset"}
// Dependencies: {}
# [doc = " Resets an iteration back to the beginning."] # [doc = ""] # [doc = " If an iterator is invalid, or if the caller wants to start over, IOIteratorReset will set the iteration back to the beginning."] # [doc = ""] # [doc = " Parameter `iterator`: An IOKit iterator handle."] # [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IOIteratorReset (iterator : io_iterator_t) { extern "C-unwind" { fn IOIteratorReset (iterator : io_iterator_t) ; } unsafe { IOIteratorReset (iterator) } }
};
}
