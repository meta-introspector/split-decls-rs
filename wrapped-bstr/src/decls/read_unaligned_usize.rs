macro_rules! read_unaligned_usize {
    () => {
        unsafe fn read_unaligned_usize (ptr : * const u8) -> usize { (ptr as * const usize) . read_unaligned () }
    };
}

read_unaligned_usize!()