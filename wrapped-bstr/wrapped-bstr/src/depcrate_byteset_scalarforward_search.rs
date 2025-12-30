// Generated macro for forward_search (function)
macro_rules! Depcrate_byteset_scalarforward_search {
() => {
// Module: crate::byteset::scalar
// Provides: {"forward_search"}
// Dependencies: {}
# [inline (always)] unsafe fn forward_search < F : Fn (u8) -> bool > (start_ptr : * const u8 , end_ptr : * const u8 , mut ptr : * const u8 , confirm : F ,) -> Option < usize > { debug_assert ! (start_ptr <= ptr) ; debug_assert ! (ptr <= end_ptr) ; while ptr < end_ptr { if confirm (* ptr) { return Some (sub (ptr , start_ptr)) ; } ptr = ptr . offset (1) ; } None }
};
}
