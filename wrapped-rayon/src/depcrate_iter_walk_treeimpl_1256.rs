// Generated macro for impl_1256 (impl)
macro_rules! Depcrate_iter_walk_treeimpl_1256 {
() => {
// Module: crate::iter::walk_tree
// Provides: {"impl_1256"}
// Dependencies: {}
impl < S , B , I > ParallelIterator for WalkTreePrefix < S , B > where S : Send , B : Fn (& S) -> I + Send + Sync , I : IntoIterator < Item = S , IntoIter : DoubleEndedIterator > , { type Item = S ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = WalkTreePrefixProducer { to_explore : once (self . initial_state) . collect () , seen : Vec :: new () , children_of : & self . children_of , } ; bridge_unindexed (producer , consumer) } }
};
}
