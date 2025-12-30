// Generated macro for impl_327 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_327 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_327"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : Ord , A : Allocator > Extend < T > for BinaryHeap < T , A > { # [inline] fn extend < I : IntoIterator < Item = T > > (& mut self , iter : I) { let guard = RebuildOnDrop { rebuild_from : self . len () , heap : self } ; guard . heap . data . extend (iter) ; } # [inline] fn extend_one (& mut self , item : T) { self . push (item) ; } # [inline] fn extend_reserve (& mut self , additional : usize) { self . reserve (additional) ; } }
};
}
