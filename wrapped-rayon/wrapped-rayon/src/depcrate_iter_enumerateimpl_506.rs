// Generated macro for impl_506 (impl)
macro_rules! Depcrate_iter_enumerateimpl_506 {
() => {
// Module: crate::iter::enumerate
// Provides: {"impl_506"}
// Dependencies: {}
impl < I > IndexedParallelIterator for Enumerate < I > where I : IndexedParallelIterator , { fn drive < C : Consumer < Self :: Item > > (self , consumer : C) -> C :: Result { bridge (self , consumer) } fn len (& self) -> usize { self . base . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { return self . base . with_producer (Callback { callback }) ; struct Callback < CB > { callback : CB , } impl < I , CB > ProducerCallback < I > for Callback < CB > where CB : ProducerCallback < (usize , I) > , { type Output = CB :: Output ; fn callback < P > (self , base : P) -> CB :: Output where P : Producer < Item = I > , { let producer = EnumerateProducer { base , offset : 0 } ; self . callback . callback (producer) } } } }
};
}
