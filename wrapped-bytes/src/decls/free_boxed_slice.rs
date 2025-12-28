macro_rules! free_boxed_slice {
    () => {
        unsafe fn free_boxed_slice (buf : * mut u8 , offset : * const u8 , len : usize) { let cap = offset . offset_from (buf) as usize + len ; dealloc (buf , Layout :: from_size_align (cap , 1) . unwrap ()) }
    };
}

free_boxed_slice!();