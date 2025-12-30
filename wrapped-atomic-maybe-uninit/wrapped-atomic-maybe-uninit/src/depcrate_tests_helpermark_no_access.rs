// Generated macro for mark_no_access (function)
macro_rules! Depcrate_tests_helpermark_no_access {
() => {
// Module: crate::tests::helper
// Provides: {"mark_no_access"}
// Dependencies: {}
# [cfg (valgrind)] # [inline (always)] pub (crate) fn mark_no_access < T : ? Sized > (a : & T) { memcheck :: mark_mem (a as * const T as * mut core :: ffi :: c_void , size_of_val (a) , memcheck :: MemState :: NoAccess ,) . unwrap () ; }
};
}
