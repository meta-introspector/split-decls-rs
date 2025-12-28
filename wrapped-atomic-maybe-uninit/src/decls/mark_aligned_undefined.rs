macro_rules! mark_aligned_undefined {
    () => {
        # [cfg (valgrind)] # [inline (always)] pub (crate) fn mark_aligned_undefined < T : ? Sized > (a : & T) { assert ! (size_of_val (a) <= 2) ; memcheck :: mark_mem ((a as * const T as * mut core :: ffi :: c_void) . map_addr (| a | a & ! 3) , 4 , memcheck :: MemState :: Undefined ,) . unwrap () ; }
    };
}

mark_aligned_undefined!();