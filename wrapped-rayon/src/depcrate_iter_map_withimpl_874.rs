// Generated macro for impl_874 (impl)
macro_rules! Depcrate_iter_map_withimpl_874 {
() => {
// Module: crate::iter::map_with
// Provides: {"impl_874"}
// Dependencies: {}
impl < I , T , F , R > IndexedParallelIterator for MapWith < I , T , F > where I : IndexedParallelIterator , T : Send + Clone , F : Fn (& mut T , I :: Item) -> R + Sync + Send , R : Send , { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { let consumer1 = MapWithConsumer :: new (consumer , self . item , & self . map_op) ; self . base . drive (consumer1) } fn len (& self) -> usize { self . base . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { return self . base . with_producer (Callback { callback , item : self . item , map_op : self . map_op , }) ; struct Callback < CB , U , F > { callback : CB , item : U , map_op : F , } impl < T , U , F , R , CB > ProducerCallback < T > for Callback < CB , U , F > where CB : ProducerCallback < R > , U : Send + Clone , F : Fn (& mut U , T) -> R + Sync , R : Send , { type Output = CB :: Output ; fn callback < P > (self , base : P) -> CB :: Output where P : Producer < Item = T > , { let producer = MapWithProducer { base , item : self . item , map_op : & self . map_op , } ; self . callback . callback (producer) } } } }
};
}
