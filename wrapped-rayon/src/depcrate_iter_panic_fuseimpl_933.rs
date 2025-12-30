// Generated macro for impl_933 (impl)
macro_rules! Depcrate_iter_panic_fuseimpl_933 {
() => {
// Module: crate::iter::panic_fuse
// Provides: {"impl_933"}
// Dependencies: {}
impl < I > IndexedParallelIterator for PanicFuse < I > where I : IndexedParallelIterator , { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { let panicked = AtomicBool :: new (false) ; let consumer1 = PanicFuseConsumer { base : consumer , fuse : Fuse (& panicked) , } ; self . base . drive (consumer1) } fn len (& self) -> usize { self . base . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { return self . base . with_producer (Callback { callback }) ; struct Callback < CB > { callback : CB , } impl < T , CB > ProducerCallback < T > for Callback < CB > where CB : ProducerCallback < T > , { type Output = CB :: Output ; fn callback < P > (self , base : P) -> CB :: Output where P : Producer < Item = T > , { let panicked = AtomicBool :: new (false) ; let producer = PanicFuseProducer { base , fuse : Fuse (& panicked) , } ; self . callback . callback (producer) } } } }
};
}
