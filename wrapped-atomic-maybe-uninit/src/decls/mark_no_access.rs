macro_rules! mark_no_access {
    () => {
        # [cfg (valgrind)] # [inline (always)] pub (crate) fn mark_no_access < T : ? Sized > (a : & T) { memcheck :: mark_mem (a as * const T as * mut core :: ffi :: c_void , size_of_val (a) , memcheck :: MemState :: NoAccess ,) . unwrap () ; }
    };
}

mark_no_access!();