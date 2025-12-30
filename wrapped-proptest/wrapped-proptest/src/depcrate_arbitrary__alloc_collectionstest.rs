// Generated macro for test (module)
macro_rules! Depcrate_arbitrary__alloc_collectionstest {
() => {
// Module: crate::arbitrary::_alloc::collections
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { no_panic_test ! (size_bounds => SizeRange , vec => Vec < u8 >, box_slice => Box < [u8] >, rc_slice => Rc < [u8] >, arc_slice => Arc < [u8] >, vec_deque => VecDeque < u8 >, linked_list => LinkedList < u8 >, btree_set => BTreeSet < u8 >, btree_map => BTreeMap < u8 , u8 >, bound => Bound < u8 >, binary_heap => BinaryHeap < u8 >, into_iter_vec => vec :: IntoIter < u8 >, into_iter_vec_deque => vec_deque :: IntoIter < u8 >, into_iter_linked_list => linked_list :: IntoIter < u8 >, into_iter_binary_heap => binary_heap :: IntoIter < u8 >, into_iter_btree_set => btree_set :: IntoIter < u8 >, into_iter_btree_map => btree_map :: IntoIter < u8 , u8 >) ; # [cfg (feature = "std")] no_panic_test ! (hash_set => HashSet < u8 >, hash_map => HashMap < u8 , u8 >, into_iter_hash_set => hash_set :: IntoIter < u8 >, into_iter_hash_map => hash_map :: IntoIter < u8 , u8 >) ; }
};
}
