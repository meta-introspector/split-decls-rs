// Generated macro for impl_1701 (impl)
macro_rules! Depcrate_vecimpl_1701 {
() => {
// Module: crate::vec
// Provides: {"impl_1701"}
// Dependencies: {}
impl < T : Send > IndexedParallelIterator for IntoIter < T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { self . vec . len () } fn with_producer < CB > (mut self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { self . vec . par_drain (..) . with_producer (callback) } }
};
}
