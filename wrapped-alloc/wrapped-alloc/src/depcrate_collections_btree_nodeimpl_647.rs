// Generated macro for impl_647 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_647 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_647"}
// Dependencies: {}
impl < 'a , K , V > Handle < NodeRef < marker :: Mut < 'a > , K , V , marker :: Internal > , marker :: KV > { pub (super) fn consider_for_balancing (self) -> BalancingContext < 'a , K , V > { let self1 = unsafe { ptr :: read (& self) } ; let self2 = unsafe { ptr :: read (& self) } ; BalancingContext { parent : self , left_child : self1 . left_edge () . descend () , right_child : self2 . right_edge () . descend () , } } }
};
}
