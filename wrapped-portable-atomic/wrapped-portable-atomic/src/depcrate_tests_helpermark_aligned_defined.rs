// Generated macro for mark_aligned_defined (function)
macro_rules! Depcrate_tests_helpermark_aligned_defined {
() => {
// Module: crate::tests::helper
// Provides: {"mark_aligned_defined"}
// Dependencies: {}
# [cfg (valgrind)] # [inline (always)] pub (crate) fn mark_aligned_defined < T : ? Sized > (a : & T) { assert ! (size_of_val (a) <= 2) ; memcheck :: mark_mem ((a as * const T as * mut core :: ffi :: c_void) . map_addr (| a | a & ! 3) , 4 , memcheck :: MemState :: Defined ,) . unwrap () ; }
};
}
