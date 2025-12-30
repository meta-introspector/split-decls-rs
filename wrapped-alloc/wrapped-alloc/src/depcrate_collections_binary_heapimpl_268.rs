// Generated macro for impl_268 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_268 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_268"}
// Dependencies: {}
# [stable (feature = "binary_heap_peek_mut" , since = "1.12.0")] impl < T : Ord , A : Allocator > Deref for PeekMut < '_ , T , A > { type Target = T ; fn deref (& self) -> & T { debug_assert ! (! self . heap . is_empty ()) ; unsafe { self . heap . data . get_unchecked (0) } } }
};
}
