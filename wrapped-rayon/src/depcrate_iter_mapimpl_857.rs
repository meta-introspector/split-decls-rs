// Generated macro for impl_857 (impl)
macro_rules! Depcrate_iter_mapimpl_857 {
() => {
// Module: crate::iter::map
// Provides: {"impl_857"}
// Dependencies: {}
impl < I , F , R > IndexedParallelIterator for Map < I , F > where I : IndexedParallelIterator , F : Fn (I :: Item) -> R + Sync + Send , R : Send , { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { let consumer1 = MapConsumer :: new (consumer , & self . map_op) ; self . base . drive (consumer1) } fn len (& self) -> usize { self . base . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { return self . base . with_producer (Callback { callback , map_op : self . map_op , }) ; struct Callback < CB , F > { callback : CB , map_op : F , } impl < T , F , R , CB > ProducerCallback < T > for Callback < CB , F > where CB : ProducerCallback < R > , F : Fn (T) -> R + Sync , R : Send , { type Output = CB :: Output ; fn callback < P > (self , base : P) -> CB :: Output where P : Producer < Item = T > , { let producer = MapProducer { base , map_op : & self . map_op , } ; self . callback . callback (producer) } } } }
};
}
