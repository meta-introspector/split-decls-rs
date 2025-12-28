macro_rules! mark_defined {
    () => {
        # [cfg (valgrind)] # [inline (always)] pub (crate) fn mark_defined < T : ? Sized > (a : & T) { memcheck :: mark_mem (a as * const T as * mut core :: ffi :: c_void , size_of_val (a) , memcheck :: MemState :: Defined ,) . unwrap () ; }
    };
}

mark_defined!()