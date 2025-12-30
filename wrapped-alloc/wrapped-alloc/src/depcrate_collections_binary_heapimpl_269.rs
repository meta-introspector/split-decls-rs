// Generated macro for impl_269 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_269 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_269"}
// Dependencies: {}
# [stable (feature = "binary_heap_peek_mut" , since = "1.12.0")] impl < T : Ord , A : Allocator > DerefMut for PeekMut < '_ , T , A > { fn deref_mut (& mut self) -> & mut T { debug_assert ! (! self . heap . is_empty ()) ; let len = self . heap . len () ; if len > 1 { unsafe { self . original_len = Some (NonZero :: new_unchecked (len)) ; self . heap . data . set_len (1) ; } } unsafe { self . heap . data . get_unchecked_mut (0) } } }
};
}
