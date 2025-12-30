// Generated macro for impl_328 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_328 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_328"}
// Dependencies: {}
# [stable (feature = "extend_ref" , since = "1.2.0")] impl < 'a , T : 'a + Ord + Copy , A : Allocator > Extend < & 'a T > for BinaryHeap < T , A > { fn extend < I : IntoIterator < Item = & 'a T > > (& mut self , iter : I) { self . extend (iter . into_iter () . cloned ()) ; } # [inline] fn extend_one (& mut self , & item : & 'a T) { self . push (item) ; } # [inline] fn extend_reserve (& mut self , additional : usize) { self . reserve (additional) ; } }
};
}
