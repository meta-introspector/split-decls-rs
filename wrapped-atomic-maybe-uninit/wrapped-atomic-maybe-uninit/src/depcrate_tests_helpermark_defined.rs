// Generated macro for mark_defined (function)
macro_rules! Depcrate_tests_helpermark_defined {
() => {
// Module: crate::tests::helper
// Provides: {"mark_defined"}
// Dependencies: {}
# [cfg (valgrind)] # [inline (always)] pub (crate) fn mark_defined < T : ? Sized > (a : & T) { memcheck :: mark_mem (a as * const T as * mut core :: ffi :: c_void , size_of_val (a) , memcheck :: MemState :: Defined ,) . unwrap () ; }
};
}
