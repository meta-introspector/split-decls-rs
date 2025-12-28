macro_rules! first_non_ascii_byte_slow {
    () => {
        # [inline (always)] unsafe fn first_non_ascii_byte_slow (start_ptr : * const u8 , end_ptr : * const u8 , mut ptr : * const u8 ,) -> usize { debug_assert ! (start_ptr <= ptr) ; debug_assert ! (ptr <= end_ptr) ; while ptr < end_ptr { if * ptr > 0x7F { return sub (ptr , start_ptr) ; } ptr = ptr . offset (1) ; } sub (end_ptr , start_ptr) }
    };
}

first_non_ascii_byte_slow!()