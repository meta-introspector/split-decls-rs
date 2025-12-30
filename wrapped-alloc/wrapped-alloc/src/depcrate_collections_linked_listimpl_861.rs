// Generated macro for impl_861 (impl)
macro_rules! Depcrate_collections_linked_listimpl_861 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_861"}
// Dependencies: {}
impl < I : IntoIterator , A : Allocator > SpecExtend < I > for LinkedList < I :: Item , A > { default fn spec_extend (& mut self , iter : I) { iter . into_iter () . for_each (move | elt | self . push_back (elt)) ; } }
};
}
