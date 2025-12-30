// Generated macro for impl_863 (impl)
macro_rules! Depcrate_collections_linked_listimpl_863 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_863"}
// Dependencies: {}
# [stable (feature = "extend_ref" , since = "1.2.0")] impl < 'a , T : 'a + Copy , A : Allocator > Extend < & 'a T > for LinkedList < T , A > { fn extend < I : IntoIterator < Item = & 'a T > > (& mut self , iter : I) { self . extend (iter . into_iter () . cloned ()) ; } # [inline] fn extend_one (& mut self , & elem : & 'a T) { self . push_back (elem) ; } }
};
}
