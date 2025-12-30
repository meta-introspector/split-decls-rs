// Generated macro for impl_633 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_633 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_633"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a > Handle < NodeRef < marker :: Mut < 'a > , K , V , marker :: Leaf > , marker :: Edge > { # [doc = " Inserts a new key-value pair between the key-value pairs to the right and left of"] # [doc = " this edge. This method splits the node if there isn't enough room."] # [doc = ""] # [doc = " Returns a dormant handle to the inserted node which can be reawakened"] # [doc = " once splitting is complete."] fn insert < A : Allocator + Clone > (self , key : K , val : V , alloc : A ,) -> (Option < SplitResult < 'a , K , V , marker :: Leaf > > , Handle < NodeRef < marker :: DormantMut , K , V , marker :: Leaf > , marker :: KV > ,) { if self . node . len () < CAPACITY { let handle = unsafe { self . insert_fit (key , val) } ; (None , handle . dormant ()) } else { let (middle_kv_idx , insertion) = splitpoint (self . idx) ; let middle = unsafe { Handle :: new_kv (self . node , middle_kv_idx) } ; let mut result = middle . split (alloc) ; let insertion_edge = match insertion { LeftOrRight :: Left (insert_idx) => unsafe { Handle :: new_edge (result . left . reborrow_mut () , insert_idx) } , LeftOrRight :: Right (insert_idx) => unsafe { Handle :: new_edge (result . right . borrow_mut () , insert_idx) } , } ; let handle = unsafe { insertion_edge . insert_fit (key , val) . dormant () } ; (Some (result) , handle) } } }
};
}
