macro_rules! block_copy {
    () => {
        fn block_copy (dst : & mut [u32] , src : & [u32] , n : usize) { dst [.. n] . copy_from_slice (& src [.. n]) ; }
    };
}

block_copy!();