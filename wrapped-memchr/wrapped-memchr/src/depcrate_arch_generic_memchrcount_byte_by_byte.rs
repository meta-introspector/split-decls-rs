// Generated macro for count_byte_by_byte (function)
macro_rules! Depcrate_arch_generic_memchrcount_byte_by_byte {
() => {
// Module: crate::arch::generic::memchr
// Provides: {"count_byte_by_byte"}
// Dependencies: {}
# [doc = " Performs a forward byte-at-a-time loop until `ptr >= end_ptr` and returns"] # [doc = " the number of times `confirm(*ptr)` returns `true`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Callers must provide valid pointers and they must satisfy `start_ptr <="] # [doc = " ptr` and `ptr <= end_ptr`."] # [inline (always)] pub (crate) unsafe fn count_byte_by_byte < F : Fn (u8) -> bool > (start : * const u8 , end : * const u8 , confirm : F ,) -> usize { debug_assert ! (start <= end) ; let mut ptr = start ; let mut count = 0 ; while ptr < end { if confirm (* ptr) { count += 1 ; } ptr = ptr . offset (1) ; } count }
};
}
