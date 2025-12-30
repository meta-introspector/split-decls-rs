// Generated macro for impl_839 (impl)
macro_rules! Depcrate_iter_lenimpl_839 {
() => {
// Module: crate::iter::len
// Provides: {"impl_839"}
// Dependencies: {}
impl < I > IndexedParallelIterator for MinLen < I > where I : IndexedParallelIterator , { fn drive < C : Consumer < Self :: Item > > (self , consumer : C) -> C :: Result { bridge (self , consumer) } fn len (& self) -> usize { self . base . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { return self . base . with_producer (Callback { callback , min : self . min , }) ; struct Callback < CB > { callback : CB , min : usize , } impl < T , CB > ProducerCallback < T > for Callback < CB > where CB : ProducerCallback < T > , { type Output = CB :: Output ; fn callback < P > (self , base : P) -> CB :: Output where P : Producer < Item = T > , { let producer = MinLenProducer { base , min : self . min , } ; self . callback . callback (producer) } } } }
};
}
