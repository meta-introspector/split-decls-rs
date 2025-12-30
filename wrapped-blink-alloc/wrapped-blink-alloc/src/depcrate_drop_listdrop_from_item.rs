// Generated macro for drop_from_item (function)
macro_rules! Depcrate_drop_listdrop_from_item {
() => {
// Module: crate::drop_list
// Provides: {"drop_from_item"}
// Dependencies: {}
# [doc = " Type-erased `core::ptr::drop_in_place` wrapper."] unsafe fn drop_from_item < T > (ptr : NonNull < Drops > , count : usize) { let ptr = ptr . cast :: < DropItem < T > > () ; let value_ptr = addr_of_mut ! ((* ptr . as_ptr ()) . value) ; core :: ptr :: drop_in_place (slice_from_raw_parts_mut (value_ptr , count)) }
};
}
