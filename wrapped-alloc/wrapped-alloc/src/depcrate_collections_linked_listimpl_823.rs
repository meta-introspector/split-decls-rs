// Generated macro for impl_823 (impl)
macro_rules! Depcrate_collections_linked_listimpl_823 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_823"}
// Dependencies: {}
impl < T > Node < T > { fn new (element : T) -> Self { Node { next : None , prev : None , element } } fn into_element < A : Allocator > (self : Box < Self , A >) -> T { self . element } }
};
}
