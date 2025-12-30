// Generated macro for impl_1016 (impl)
macro_rules! Depcrate_iter_repeatimpl_1016 {
() => {
// Module: crate::iter::repeat
// Provides: {"impl_1016"}
// Dependencies: {}
impl < T > IndexedParallelIterator for RepeatN < T > where T : Clone + Send , { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { callback . callback (self . inner) } fn len (& self) -> usize { self . inner . len () } }
};
}
