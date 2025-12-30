// Generated macro for impl_957 (impl)
macro_rules! Depcrate_iter_par_bridgeimpl_957 {
() => {
// Module: crate::iter::par_bridge
// Provides: {"impl_957"}
// Dependencies: {}
impl < Iter > ParallelIterator for IterBridge < Iter > where Iter : Iterator < Item : Send > + Send , { type Item = Iter :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let num_threads = current_num_threads () ; let threads_started : Vec < _ > = (0 .. num_threads) . map (| _ | AtomicBool :: new (false)) . collect () ; bridge_unindexed (& IterParallelProducer { split_count : AtomicUsize :: new (num_threads) , iter : Mutex :: new (self . iter . fuse ()) , threads_started : & threads_started , } , consumer ,) } }
};
}
