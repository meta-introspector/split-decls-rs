// Generated macro for impl_49 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_49 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_49"}
// Dependencies: {}
impl < T : Ord + Send > IndexedParallelIterator for Drain < '_ , T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { self . heap . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { super :: DrainGuard :: new (self . heap) . par_drain (..) . with_producer (callback) } }
};
}
