// Generated macro for reverse_search (function)
macro_rules! Depcrate_byteset_scalarreverse_search {
() => {
// Module: crate::byteset::scalar
// Provides: {"reverse_search"}
// Dependencies: {}
# [inline (always)] unsafe fn reverse_search < F : Fn (u8) -> bool > (start_ptr : * const u8 , end_ptr : * const u8 , mut ptr : * const u8 , confirm : F ,) -> Option < usize > { debug_assert ! (start_ptr <= ptr) ; debug_assert ! (ptr <= end_ptr) ; while ptr > start_ptr { ptr = ptr . offset (- 1) ; if confirm (* ptr) { return Some (sub (ptr , start_ptr)) ; } } None }
};
}
