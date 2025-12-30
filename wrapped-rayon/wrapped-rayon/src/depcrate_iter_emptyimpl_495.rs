// Generated macro for impl_495 (impl)
macro_rules! Depcrate_iter_emptyimpl_495 {
() => {
// Module: crate::iter::empty
// Provides: {"impl_495"}
// Dependencies: {}
impl < T : Send > IndexedParallelIterator for Empty < T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { consumer . into_folder () . complete () } fn len (& self) -> usize { 0 } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { callback . callback (EmptyProducer (PhantomData)) } }
};
}
