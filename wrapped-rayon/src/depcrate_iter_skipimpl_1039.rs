// Generated macro for impl_1039 (impl)
macro_rules! Depcrate_iter_skipimpl_1039 {
() => {
// Module: crate::iter::skip
// Provides: {"impl_1039"}
// Dependencies: {}
impl < I > IndexedParallelIterator for Skip < I > where I : IndexedParallelIterator , { fn len (& self) -> usize { self . base . len () - self . n } fn drive < C : Consumer < Self :: Item > > (self , consumer : C) -> C :: Result { bridge (self , consumer) } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { return self . base . with_producer (Callback { callback , n : self . n , }) ; struct Callback < CB > { callback : CB , n : usize , } impl < T , CB > ProducerCallback < T > for Callback < CB > where CB : ProducerCallback < T > , { type Output = CB :: Output ; fn callback < P > (self , base : P) -> CB :: Output where P : Producer < Item = T > , { crate :: in_place_scope (| scope | { let Self { callback , n } = self ; let (before_skip , after_skip) = base . split_at (n) ; scope . spawn (move | _ | bridge_producer_consumer (n , before_skip , NoopConsumer)) ; callback . callback (after_skip) }) } } } }
};
}
