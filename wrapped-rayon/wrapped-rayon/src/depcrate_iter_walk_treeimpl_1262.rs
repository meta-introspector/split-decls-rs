// Generated macro for impl_1262 (impl)
macro_rules! Depcrate_iter_walk_treeimpl_1262 {
() => {
// Module: crate::iter::walk_tree
// Provides: {"impl_1262"}
// Dependencies: {}
impl < S , B , I > ParallelIterator for WalkTreePostfix < S , B > where S : Send , B : Fn (& S) -> I + Send + Sync , I : IntoIterator < Item = S > , { type Item = S ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = WalkTreePostfixProducer { to_explore : once (self . initial_state) . collect () , seen : Vec :: new () , children_of : & self . children_of , } ; bridge_unindexed (producer , consumer) } }
};
}
