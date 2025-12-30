// Generated macro for mark_aligned_undefined (function)
macro_rules! Depcrate_tests_helpermark_aligned_undefined {
() => {
// Module: crate::tests::helper
// Provides: {"mark_aligned_undefined"}
// Dependencies: {}
# [cfg (valgrind)] # [inline (always)] pub (crate) fn mark_aligned_undefined < T : ? Sized > (a : & T) { assert ! (size_of_val (a) <= 2) ; memcheck :: mark_mem ((a as * const T as * mut core :: ffi :: c_void) . map_addr (| a | a & ! 3) , 4 , memcheck :: MemState :: Undefined ,) . unwrap () ; }
};
}
