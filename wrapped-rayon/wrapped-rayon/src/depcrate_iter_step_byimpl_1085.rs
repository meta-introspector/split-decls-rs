// Generated macro for impl_1085 (impl)
macro_rules! Depcrate_iter_step_byimpl_1085 {
() => {
// Module: crate::iter::step_by
// Provides: {"impl_1085"}
// Dependencies: {}
impl < I > IndexedParallelIterator for StepBy < I > where I : IndexedParallelIterator , { fn drive < C : Consumer < Self :: Item > > (self , consumer : C) -> C :: Result { bridge (self , consumer) } fn len (& self) -> usize { self . base . len () . div_ceil (self . step) } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { let len = self . base . len () ; return self . base . with_producer (Callback { callback , step : self . step , len , }) ; struct Callback < CB > { callback : CB , step : usize , len : usize , } impl < T , CB > ProducerCallback < T > for Callback < CB > where CB : ProducerCallback < T > , { type Output = CB :: Output ; fn callback < P > (self , base : P) -> CB :: Output where P : Producer < Item = T > , { let producer = StepByProducer { base , step : self . step , len : self . len , } ; self . callback . callback (producer) } } } }
};
}
