// Generated macro for impl_149 (impl)
macro_rules! Depcrate_collections_vec_dequeimpl_149 {
() => {
// Module: crate::collections::vec_deque
// Provides: {"impl_149"}
// Dependencies: {}
impl < T : Send > IndexedParallelIterator for Drain < '_ , T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { self . range . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { super :: DrainGuard :: new (self . deque) . par_drain (self . range . clone ()) . with_producer (callback) } }
};
}
