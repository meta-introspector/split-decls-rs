// Generated macro for impl_1593 (impl)
macro_rules! Depcrate_sliceimpl_1593 {
() => {
// Module: crate::slice
// Provides: {"impl_1593"}
// Dependencies: {}
impl < T : Send > IndexedParallelIterator for IterMut < '_ , T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { self . slice . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { callback . callback (IterMutProducer { slice : self . slice }) } }
};
}
