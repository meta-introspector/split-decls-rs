// Generated macro for impl_391 (impl)
macro_rules! Depcrate_iter_chunksimpl_391 {
() => {
// Module: crate::iter::chunks
// Provides: {"impl_391"}
// Dependencies: {}
impl < I > IndexedParallelIterator for Chunks < I > where I : IndexedParallelIterator , { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { self . i . len () . div_ceil (self . size) } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { let len = self . i . len () ; return self . i . with_producer (Callback { size : self . size , len , callback , }) ; struct Callback < CB > { size : usize , len : usize , callback : CB , } impl < T , CB > ProducerCallback < T > for Callback < CB > where CB : ProducerCallback < Vec < T > > , { type Output = CB :: Output ; fn callback < P > (self , base : P) -> CB :: Output where P : Producer < Item = T > , { let producer = ChunkProducer :: new (self . size , self . len , base , Vec :: from_iter) ; self . callback . callback (producer) } } } }
};
}
