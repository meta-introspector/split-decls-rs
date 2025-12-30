// Generated macro for impl_856 (impl)
macro_rules! Depcrate_collections_linked_listimpl_856 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_856"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T > FromIterator < T > for LinkedList < T > { fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> Self { let mut list = Self :: new () ; list . extend (iter) ; list } }
};
}
