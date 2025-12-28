macro_rules! rev_byte_by_byte {
    () => {
        # [doc = " Performs a reverse byte-at-a-time loop until either `ptr < start_ptr` or"] # [doc = " until `confirm(*ptr)` returns `true`. If the former occurs, then `None` is"] # [doc = " returned. If the latter occurs, then the pointer at which `confirm` returns"] # [doc = " `true` is returned."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Callers must provide valid pointers and they must satisfy `start_ptr <="] # [doc = " ptr` and `ptr <= end_ptr`."] # [inline (always)] pub (crate) unsafe fn rev_byte_by_byte < F : Fn (u8) -> bool > (start : * const u8 , end : * const u8 , confirm : F ,) -> Option < * const u8 > { debug_assert ! (start <= end) ; let mut ptr = end ; while ptr > start { ptr = ptr . offset (- 1) ; if confirm (* ptr) { return Some (ptr) ; } } None }
    };
}

rev_byte_by_byte!()