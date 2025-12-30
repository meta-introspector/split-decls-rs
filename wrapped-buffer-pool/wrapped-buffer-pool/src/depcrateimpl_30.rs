// Generated macro for impl_30 (impl)
macro_rules! Depcrateimpl_30 {
() => {
// Module: crate
// Provides: {"impl_30"}
// Dependencies: {}
impl < T > QueueShard < T > { const fn new (trim : usize , max : usize) -> Self { QueueShard { queue : SegQueue :: new () , elem_cnt : AtomicUsize :: new (0) , trim , max , } } }
};
}
