// Generated macro for impl_1355 (impl)
macro_rules! Depcrate_rangeimpl_1355 {
() => {
// Module: crate::range
// Provides: {"impl_1355"}
// Dependencies: {}
impl < T : IndexedRangeInteger > IndexedParallelIterator for Iter < T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < T > , { T :: drive (self , consumer) } # [inline] fn len (& self) -> usize { T :: len (self) } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < T > , { T :: with_producer (self , callback) } }
};
}
