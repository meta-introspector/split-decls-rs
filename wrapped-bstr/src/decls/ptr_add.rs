macro_rules! ptr_add {
    () => {
        # [doc = " Increment the given pointer by the given amount."] unsafe fn ptr_add (ptr : * const u8 , amt : usize) -> * const u8 { ptr . add (amt) }
    };
}

ptr_add!();