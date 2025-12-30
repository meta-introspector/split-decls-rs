// Generated macro for impl_500 (impl)
macro_rules! Depcrate_vector_rayonimpl_500 {
() => {
// Module: crate::vector::rayon
// Provides: {"impl_500"}
// Dependencies: {}
impl < 'a , A > IndexedParallelIterator for ParIter < 'a , A > where A : Clone + Send + Sync + 'a , { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { self . focus . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { callback . callback (VectorProducer { focus : self . focus }) } }
};
}
