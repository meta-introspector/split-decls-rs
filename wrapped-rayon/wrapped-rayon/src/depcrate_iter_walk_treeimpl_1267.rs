// Generated macro for impl_1267 (impl)
macro_rules! Depcrate_iter_walk_treeimpl_1267 {
() => {
// Module: crate::iter::walk_tree
// Provides: {"impl_1267"}
// Dependencies: {}
impl < S , B , I > ParallelIterator for WalkTree < S , B > where S : Send , B : Fn (& S) -> I + Send + Sync , I : IntoIterator < Item = S , IntoIter : DoubleEndedIterator > + Send , { type Item = S ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . 0 . drive_unindexed (consumer) } }
};
}
