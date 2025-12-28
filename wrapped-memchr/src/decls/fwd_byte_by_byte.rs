macro_rules! fwd_byte_by_byte {
    () => {
        # [doc = " Performs a forward byte-at-a-time loop until either `ptr >= end_ptr` or"] # [doc = " until `confirm(*ptr)` returns `true`. If the former occurs, then `None` is"] # [doc = " returned. If the latter occurs, then the pointer at which `confirm` returns"] # [doc = " `true` is returned."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Callers must provide valid pointers and they must satisfy `start_ptr <="] # [doc = " ptr` and `ptr <= end_ptr`."] # [inline (always)] pub (crate) unsafe fn fwd_byte_by_byte < F : Fn (u8) -> bool > (start : * const u8 , end : * const u8 , confirm : F ,) -> Option < * const u8 > { debug_assert ! (start <= end) ; let mut ptr = start ; while ptr < end { if confirm (* ptr) { return Some (ptr) ; } ptr = ptr . offset (1) ; } None }
    };
}

fwd_byte_by_byte!();