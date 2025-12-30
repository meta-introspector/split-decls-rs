// Generated macro for impl_1075 (impl)
macro_rules! Depcrate_iter_splitterimpl_1075 {
() => {
// Module: crate::iter::splitter
// Provides: {"impl_1075"}
// Dependencies: {}
impl < D , S > ParallelIterator for Split < D , S > where D : Send , S : Fn (D) -> (D , Option < D >) + Sync + Send , { type Item = D ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = SplitProducer { data : self . data , splitter : & self . splitter , } ; bridge_unindexed (producer , consumer) } }
};
}
