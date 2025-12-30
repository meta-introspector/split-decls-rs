// Generated macro for impl_1588 (impl)
macro_rules! Depcrate_sliceimpl_1588 {
() => {
// Module: crate::slice
// Provides: {"impl_1588"}
// Dependencies: {}
impl < T : Sync > IndexedParallelIterator for Windows < '_ , T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { assert ! (self . window_size >= 1) ; self . slice . len () . saturating_sub (self . window_size - 1) } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { callback . callback (WindowsProducer { window_size : self . window_size , slice : self . slice , }) } }
};
}
