macro_rules! ptr_sub {
    () => {
        # [doc = " Decrement the given pointer by the given amount."] unsafe fn ptr_sub (ptr : * const u8 , amt : usize) -> * const u8 { ptr . sub (amt) }
    };
}

ptr_sub!()