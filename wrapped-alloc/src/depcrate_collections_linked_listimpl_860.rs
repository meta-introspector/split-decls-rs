// Generated macro for impl_860 (impl)
macro_rules! Depcrate_collections_linked_listimpl_860 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_860"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , A : Allocator > Extend < T > for LinkedList < T , A > { fn extend < I : IntoIterator < Item = T > > (& mut self , iter : I) { < Self as SpecExtend < I > > :: spec_extend (self , iter) ; } # [inline] fn extend_one (& mut self , elem : T) { self . push_back (elem) ; } }
};
}
