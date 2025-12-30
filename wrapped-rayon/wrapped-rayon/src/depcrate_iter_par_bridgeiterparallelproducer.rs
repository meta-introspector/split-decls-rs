// Generated macro for IterParallelProducer (struct)
macro_rules! Depcrate_iter_par_bridgeIterParallelProducer {
() => {
// Module: crate::iter::par_bridge
// Provides: {"IterParallelProducer"}
// Dependencies: {}
struct IterParallelProducer < 'a , Iter > { split_count : AtomicUsize , iter : Mutex < std :: iter :: Fuse < Iter > > , threads_started : & 'a [AtomicBool] , }
};
}
