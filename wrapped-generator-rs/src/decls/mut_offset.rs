macro_rules! mut_offset {
    () => {
        # [inline] # [allow (dead_code)] fn mut_offset < T > (ptr : * mut T , count : isize) -> * mut T { unsafe { ptr . offset (count) } }
    };
}

mut_offset!()