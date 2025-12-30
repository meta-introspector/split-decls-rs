// Generated macro for impl_1582 (impl)
macro_rules! Depcrate_sliceimpl_1582 {
() => {
// Module: crate::slice
// Provides: {"impl_1582"}
// Dependencies: {}
impl < T : Sync > IndexedParallelIterator for Iter < '_ , T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { self . slice . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { callback . callback (IterProducer { slice : self . slice }) } }
};
}
