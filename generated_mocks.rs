// Generated mocks for src/temp_processed_.._rust_library_alloc_src_collections_btree_append.rs
mkmod! { merge_iter {
    impl <I: Iterator> Clone for MergeIterInner<I> {
        pub fn clone(&self) { unimplemented!() }
    }
    impl <I: Iterator> Debug for MergeIterInner<I> {
        pub fn fmt(&self) { unimplemented!() }
    }
    impl <I: Iterator> MergeIterInner<I> {
    }
}
mkmod! { node {
    pub struct LeafNode<K,;
    pub struct InternalNode<K,;
    pub struct Dropper<'a,;
    impl <K, V> LeafNode<K, V> {
    }
    impl <K, V> InternalNode<K, V> {
    }
    impl <'a, K: 'a, V: 'a, Type> Copy for NodeRef<marker::Immut<'a>, K, V, Type> {
    impl <'a, K: 'a, V: 'a, Type> Clone for NodeRef<marker::Immut<'a>, K, V, Type> {
        pub fn clone(&self) { unimplemented!() }
    }
    impl <K, V> NodeRef<marker::Owned, K, V, marker::Leaf> {
    }
    impl <K, V> NodeRef<marker::Owned, K, V, marker::Internal> {
        pub fn new_internal<A: Allocator + Clone>(&self) { unimplemented!() }
    }
    impl <BorrowType, K, V> NodeRef<BorrowType, K, V, marker::Internal> {
        pub fn from_internal(&self) { unimplemented!() }
    }
    impl <BorrowType, K, V> NodeRef<BorrowType, K, V, marker::Internal> {
        pub fn as_internal_ptr(&self) { unimplemented!() }
    }
    impl <'a, K, V> NodeRef<marker::Mut<'a>, K, V, marker::Internal> {
        pub fn as_internal_mut(&self) { unimplemented!() }
    }
    impl <BorrowType, K, V, Type> NodeRef<BorrowType, K, V, Type> {
    }
    impl <BorrowType: marker::BorrowType, K, V, Type> NodeRef<BorrowType, K, V, Type> {
    }
    impl <BorrowType, K, V, Type> NodeRef<BorrowType, K, V, Type> {
        pub fn eq(&self) { unimplemented!() }
    }
    impl <'a, K: 'a, V: 'a, Type> NodeRef<marker::Immut<'a>, K, V, Type> {
        pub fn into_leaf(&self) { unimplemented!() }
    }
    impl <K, V> NodeRef<marker::Dying, K, V, marker::LeafOrInternal> {
    }
    impl <'a, K, V, Type> NodeRef<marker::Mut<'a>, K, V, Type> {
    }
    impl <K, V, Type> NodeRef<marker::DormantMut, K, V, Type> {
    }
    impl <K, V, Type> NodeRef<marker::Dying, K, V, Type> {
        pub fn as_leaf_dying(&self) { unimplemented!() }
    }
    impl <'a, K: 'a, V: 'a, Type> NodeRef<marker::Mut<'a>, K, V, Type> {
    }
    impl <'a, K: 'a, V: 'a> NodeRef<marker::Mut<'a>, K, V, marker::Internal> {
    }
    impl <'a, K, V, Type> NodeRef<marker::ValMut<'a>, K, V, Type> {
    }
    impl <'a, K: 'a, V: 'a, Type> NodeRef<marker::Mut<'a>, K, V, Type> {
    }
    impl <'a, K, V> NodeRef<marker::Mut<'a>, K, V, marker::Internal> {
    }
    impl <'a, K: 'a, V: 'a> NodeRef<marker::Mut<'a>, K, V, marker::LeafOrInternal> {
        pub fn set_parent_link(&self) { unimplemented!() }
    }
    impl <K, V> NodeRef<marker::Owned, K, V, marker::LeafOrInternal> {
        pub fn clear_parent_link(&self) { unimplemented!() }
    }
    impl <K, V> NodeRef<marker::Owned, K, V, marker::LeafOrInternal> {
    }
    impl <K, V, Type> NodeRef<marker::Owned, K, V, Type> {
    }
    impl <'a, K: 'a, V: 'a> NodeRef<marker::Mut<'a>, K, V, marker::Leaf> {
    }
    impl <'a, K: 'a, V: 'a> NodeRef<marker::Mut<'a>, K, V, marker::Internal> {
    }
    impl <BorrowType, K, V> NodeRef<BorrowType, K, V, marker::Leaf> {
    }
    impl <BorrowType, K, V> NodeRef<BorrowType, K, V, marker::Internal> {
    }
    impl <BorrowType, K, V> NodeRef<BorrowType, K, V, marker::LeafOrInternal> {
    }
    impl <'a, K, V> NodeRef<marker::Mut<'a>, K, V, marker::LeafOrInternal> {
    }
    impl <Node: Copy, Type> Copy for Handle<Node, Type> {
    impl <Node: Copy, Type> Clone for Handle<Node, Type> {
        pub fn clone(&self) { unimplemented!() }
    }
    impl <Node, Type> Handle<Node, Type> {
    }
    impl <BorrowType, K, V, NodeType> Handle<NodeRef<BorrowType, K, V, NodeType>, marker::KV> {
    }
    impl <BorrowType, K, V, NodeType, HandleType> PartialEq {
        pub fn eq(&self) { unimplemented!() }
    }
    impl <BorrowType, K, V, NodeType, HandleType> {
    }
    impl <'a, K, V, NodeType, HandleType> Handle<NodeRef<marker::Mut<'a>, K, V, NodeType>, HandleType> {
    }
    impl <K, V, NodeType, HandleType> Handle<NodeRef<marker::DormantMut, K, V, NodeType>, HandleType> {
    }
    impl <BorrowType, K, V, NodeType> Handle<NodeRef<BorrowType, K, V, NodeType>, marker::Edge> {
    }
    impl <'a, K: 'a, V: 'a> Handle<NodeRef<marker::Mut<'a>, K, V, marker::Leaf>, marker::Edge> {
    }
    impl <'a, K: 'a, V: 'a> Handle<NodeRef<marker::Mut<'a>, K, V, marker::Leaf>, marker::Edge> {
        pub fn insert<A: Allocator + Clone>(&self) { unimplemented!() }
    }
    impl <'a, K, V> Handle<NodeRef<marker::Mut<'a>, K, V, marker::Internal>, marker::Edge> {
        pub fn correct_parent_link(&self) { unimplemented!() }
    }
    impl <'a, K: 'a, V: 'a> Handle<NodeRef<marker::Mut<'a>, K, V, marker::Internal>, marker::Edge> {
        pub fn insert_fit(&self) { unimplemented!() }
    }
    impl <'a, K: 'a, V: 'a> Handle<NodeRef<marker::Mut<'a>, K, V, marker::Leaf>, marker::Edge> {
    }
    impl <BorrowType: marker::BorrowType, K, V> {
    }
    impl <'a, K: 'a, V: 'a, NodeType> Handle<NodeRef<marker::Immut<'a>, K, V, NodeType>, marker::KV> {
    }
    impl <'a, K: 'a, V: 'a, NodeType> Handle<NodeRef<marker::Mut<'a>, K, V, NodeType>, marker::KV> {
    }
    impl <'a, K, V, NodeType> Handle<NodeRef<marker::ValMut<'a>, K, V, NodeType>, marker::KV> {
    }
    impl <'a, K: 'a, V: 'a, NodeType> Handle<NodeRef<marker::Mut<'a>, K, V, NodeType>, marker::KV> {
    }
    impl <K, V, NodeType> Handle<NodeRef<marker::Dying, K, V, NodeType>, marker::KV> {
    }
    impl <T> Drop for Dropper<'_, T> {
        pub fn drop(&self) { unimplemented!() }
    }
    impl <'a, K: 'a, V: 'a, NodeType> Handle<NodeRef<marker::Mut<'a>, K, V, NodeType>, marker::KV> {
        pub fn split_leaf_data(&self) { unimplemented!() }
    }
    impl <'a, K: 'a, V: 'a> Handle<NodeRef<marker::Mut<'a>, K, V, marker::Leaf>, marker::KV> {
    }
    impl <'a, K: 'a, V: 'a> Handle<NodeRef<marker::Mut<'a>, K, V, marker::Internal>, marker::KV> {
    }
    impl <'a, K, V> Handle<NodeRef<marker::Mut<'a>, K, V, marker::Internal>, marker::KV> {
    }
    impl <'a, K, V> NodeRef<marker::Mut<'a>, K, V, marker::LeafOrInternal> {
    }
    impl <'a, K, V> BalancingContext<'a, K, V> {
    }
    impl <'a, K: 'a, V: 'a> BalancingContext<'a, K, V> {
        // fn do_merge<
    }
    impl <BorrowType, K, V> Handle<NodeRef<BorrowType, K, V, marker::Leaf>, marker::Edge> {
    }
    impl <BorrowType, K, V> Handle<NodeRef<BorrowType, K, V, marker::Internal>, marker::Edge> {
    }
    impl <BorrowType, K, V> Handle<NodeRef<BorrowType, K, V, marker::Leaf>, marker::KV> {
    }
    impl <BorrowType, K, V, Type> Handle<NodeRef<BorrowType, K, V, marker::LeafOrInternal>, Type> {
    }
    impl <'a, K, V, Type> Handle<NodeRef<marker::Mut<'a>, K, V, marker::LeafOrInternal>, Type> {
    }
    impl <'a, K, V> Handle<NodeRef<marker::Mut<'a>, K, V, marker::LeafOrInternal>, marker::Edge> {
    }
    impl <'a, K, V> SplitResult<'a, K, V, marker::Leaf> {
    }
    impl <'a, K, V> SplitResult<'a, K, V, marker::Internal> {
    }
    impl BorrowType for Owned {
    }
    impl BorrowType for Dying {
    impl <'a> BorrowType for Immut<'a> {
    impl <'a> BorrowType for Mut<'a> {
    impl <'a> BorrowType for ValMut<'a> {
    impl BorrowType for DormantMut {
    }
}
