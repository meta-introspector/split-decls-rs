// Generated declarations for node module
// Extracted from: ../rust/library/alloc/src/collections/btree/node.rs

const B: usize = 6;
const KV_IDX_CENTER: usize = B - 1;
const EDGE_IDX_LEFT_OF_CENTER: usize = B - 1;
const EDGE_IDX_RIGHT_OF_CENTER: usize = B;
struct LeafNode<K, V> {
impl<K, V> LeafNode<K, V> {
    fn new<A: Allocator + Clone>(alloc: A) -> Box<Self, A> { unimplemented!() }
}
struct InternalNode<K, V> {
impl<K, V> InternalNode<K, V> {
}
impl<'a, K: 'a, V: 'a, Type> Clone for NodeRef<marker::Immut<'a>, K, V, Type> {
    fn clone(&self) -> Self { unimplemented!() }
}
impl<K, V> NodeRef<marker::Owned, K, V, marker::Leaf> {
    fn from_new_leaf<A: Allocator + Clone>(leaf: Box<LeafNode<K, V>, A>) -> Self { unimplemented!() }
}
impl<K, V> NodeRef<marker::Owned, K, V, marker::Internal> {
    fn new_internal<A: Allocator + Clone>(child: Root<K, V>, alloc: A) -> Self { unimplemented!() }
}
impl<BorrowType, K, V> NodeRef<BorrowType, K, V, marker::Internal> {
    fn from_internal(node: NonNull<InternalNode<K, V>>, height: usize) -> Self { unimplemented!() }
}
impl<BorrowType, K, V> NodeRef<BorrowType, K, V, marker::Internal> {
    fn as_internal_ptr(this: &Self) -> *mut InternalNode<K, V> { unimplemented!() }
}
impl<'a, K, V> NodeRef<marker::Mut<'a>, K, V, marker::Internal> {
    fn as_internal_mut(&mut self) -> &mut InternalNode<K, V> { unimplemented!() }
}
impl<BorrowType, K, V, Type> NodeRef<BorrowType, K, V, Type> {
    fn as_leaf_ptr(this: &Self) -> *mut LeafNode<K, V> { unimplemented!() }
}
impl<BorrowType: marker::BorrowType, K, V, Type> NodeRef<BorrowType, K, V, Type> {
}
impl<BorrowType, K, V, Type> NodeRef<BorrowType, K, V, Type> {
    fn eq(&self, other: &Self) -> bool { unimplemented!() }
}
impl<'a, K: 'a, V: 'a, Type> NodeRef<marker::Immut<'a>, K, V, Type> {
    fn into_leaf(self) -> &'a LeafNode<K, V> { unimplemented!() }
}
impl<K, V> NodeRef<marker::Dying, K, V, marker::LeafOrInternal> {
}
impl<'a, K, V, Type> NodeRef<marker::Mut<'a>, K, V, Type> {
    fn as_leaf_mut(&mut self) -> &mut LeafNode<K, V> { unimplemented!() }
    fn into_leaf_mut(mut self) -> &'a mut LeafNode<K, V> { unimplemented!() }
}
impl<K, V, Type> NodeRef<marker::DormantMut, K, V, Type> {
}
impl<K, V, Type> NodeRef<marker::Dying, K, V, Type> {
    fn as_leaf_dying(&mut self) -> &mut LeafNode<K, V> { unimplemented!() }
}
impl<'a, K: 'a, V: 'a, Type> NodeRef<marker::Mut<'a>, K, V, Type> {
}
impl<'a, K: 'a, V: 'a> NodeRef<marker::Mut<'a>, K, V, marker::Internal> {
}
impl<'a, K, V, Type> NodeRef<marker::ValMut<'a>, K, V, Type> {
}
impl<'a, K: 'a, V: 'a, Type> NodeRef<marker::Mut<'a>, K, V, Type> {
}
impl<'a, K, V> NodeRef<marker::Mut<'a>, K, V, marker::Internal> {
    fn correct_all_childrens_parent_links(&mut self) { unimplemented!() }
}
impl<'a, K: 'a, V: 'a> NodeRef<marker::Mut<'a>, K, V, marker::LeafOrInternal> {
    fn set_parent_link(&mut self, parent: NonNull<InternalNode<K, V>>, parent_idx: usize) { unimplemented!() }
}
impl<K, V> NodeRef<marker::Owned, K, V, marker::LeafOrInternal> {
    fn clear_parent_link(&mut self) { unimplemented!() }
}
impl<K, V> NodeRef<marker::Owned, K, V, marker::LeafOrInternal> {
}
impl<K, V, Type> NodeRef<marker::Owned, K, V, Type> {
}
impl<'a, K: 'a, V: 'a> NodeRef<marker::Mut<'a>, K, V, marker::Leaf> {
}
impl<'a, K: 'a, V: 'a> NodeRef<marker::Mut<'a>, K, V, marker::Internal> {
}
impl<BorrowType, K, V> NodeRef<BorrowType, K, V, marker::Leaf> {
}
impl<BorrowType, K, V> NodeRef<BorrowType, K, V, marker::Internal> {
}
impl<BorrowType, K, V> NodeRef<BorrowType, K, V, marker::LeafOrInternal> {
}
impl<'a, K, V> NodeRef<marker::Mut<'a>, K, V, marker::LeafOrInternal> {
}
impl<Node: Copy, Type> Clone for Handle<Node, Type> {
    fn clone(&self) -> Self { unimplemented!() }
}
impl<Node, Type> Handle<Node, Type> {
}
impl<BorrowType, K, V, NodeType> Handle<NodeRef<BorrowType, K, V, NodeType>, marker::KV> {
}
impl<BorrowType, K, V, NodeType, HandleType> PartialEq
}
impl<BorrowType, K, V, NodeType, HandleType>
}
impl<'a, K, V, NodeType, HandleType> Handle<NodeRef<marker::Mut<'a>, K, V, NodeType>, HandleType> {
}
impl<K, V, NodeType, HandleType> Handle<NodeRef<marker::DormantMut, K, V, NodeType>, HandleType> {
}
impl<BorrowType, K, V, NodeType> Handle<NodeRef<BorrowType, K, V, NodeType>, marker::Edge> {
}
impl<'a, K: 'a, V: 'a> Handle<NodeRef<marker::Mut<'a>, K, V, marker::Leaf>, marker::Edge> {
}
impl<'a, K: 'a, V: 'a> Handle<NodeRef<marker::Mut<'a>, K, V, marker::Leaf>, marker::Edge> {
    // fn insert<A: Allocator + Clone>(
}
impl<'a, K, V> Handle<NodeRef<marker::Mut<'a>, K, V, marker::Internal>, marker::Edge> {
    fn correct_parent_link(self) { unimplemented!() }
}
impl<'a, K: 'a, V: 'a> Handle<NodeRef<marker::Mut<'a>, K, V, marker::Internal>, marker::Edge> {
    fn insert_fit(&mut self, key: K, val: V, edge: Root<K, V>) { unimplemented!() }
    // fn insert<A: Allocator + Clone>(
}
impl<'a, K: 'a, V: 'a> Handle<NodeRef<marker::Mut<'a>, K, V, marker::Leaf>, marker::Edge> {
}
impl<BorrowType: marker::BorrowType, K, V>
}
        const {
impl<'a, K: 'a, V: 'a, NodeType> Handle<NodeRef<marker::Immut<'a>, K, V, NodeType>, marker::KV> {
}
impl<'a, K: 'a, V: 'a, NodeType> Handle<NodeRef<marker::Mut<'a>, K, V, NodeType>, marker::KV> {
}
impl<'a, K, V, NodeType> Handle<NodeRef<marker::ValMut<'a>, K, V, NodeType>, marker::KV> {
}
impl<'a, K: 'a, V: 'a, NodeType> Handle<NodeRef<marker::Mut<'a>, K, V, NodeType>, marker::KV> {
}
        impl<T> Drop for Dropper<'_, T> {
    fn drop(&mut self) { unimplemented!() }
}
impl<'a, K: 'a, V: 'a, NodeType> Handle<NodeRef<marker::Mut<'a>, K, V, NodeType>, marker::KV> {
    fn split_leaf_data(&mut self, new_node: &mut LeafNode<K, V>) -> (K, V) { unimplemented!() }
}
impl<'a, K: 'a, V: 'a> Handle<NodeRef<marker::Mut<'a>, K, V, marker::Leaf>, marker::KV> {
}
impl<'a, K: 'a, V: 'a> Handle<NodeRef<marker::Mut<'a>, K, V, marker::Internal>, marker::KV> {
}
impl<'a, K, V> Handle<NodeRef<marker::Mut<'a>, K, V, marker::Internal>, marker::KV> {
}
impl<'a, K, V> NodeRef<marker::Mut<'a>, K, V, marker::LeafOrInternal> {
}
impl<'a, K, V> BalancingContext<'a, K, V> {
}
impl<'a, K: 'a, V: 'a> BalancingContext<'a, K, V> {
    // fn do_merge<
}
impl<BorrowType, K, V> Handle<NodeRef<BorrowType, K, V, marker::Leaf>, marker::Edge> {
}
impl<BorrowType, K, V> Handle<NodeRef<BorrowType, K, V, marker::Internal>, marker::Edge> {
}
impl<BorrowType, K, V> Handle<NodeRef<BorrowType, K, V, marker::Leaf>, marker::KV> {
}
impl<BorrowType, K, V, Type> Handle<NodeRef<BorrowType, K, V, marker::LeafOrInternal>, Type> {
}
impl<'a, K, V, Type> Handle<NodeRef<marker::Mut<'a>, K, V, marker::LeafOrInternal>, Type> {
}
impl<'a, K, V> Handle<NodeRef<marker::Mut<'a>, K, V, marker::LeafOrInternal>, marker::Edge> {
}
impl<'a, K, V> SplitResult<'a, K, V, marker::Leaf> {
}
impl<'a, K, V> SplitResult<'a, K, V, marker::Internal> {
}
        const TRAVERSAL_PERMIT: bool = true;
    impl BorrowType for Owned {
}
    impl BorrowType for DormantMut {}
}
