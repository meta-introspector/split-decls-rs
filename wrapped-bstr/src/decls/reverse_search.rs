macro_rules! reverse_search {
    () => {
        # [inline (always)] unsafe fn reverse_search < F : Fn (u8) -> bool > (start_ptr : * const u8 , end_ptr : * const u8 , mut ptr : * const u8 , confirm : F ,) -> Option < usize > { debug_assert ! (start_ptr <= ptr) ; debug_assert ! (ptr <= end_ptr) ; while ptr > start_ptr { ptr = ptr . offset (- 1) ; if confirm (* ptr) { return Some (sub (ptr , start_ptr)) ; } } None }
    };
}

reverse_search!()