// Generated macro for impl_810 (impl)
macro_rules! Depcrate_iter_interleave_shortestimpl_810 {
() => {
// Module: crate::iter::interleave_shortest
// Provides: {"impl_810"}
// Dependencies: {}
impl < I , J > IndexedParallelIterator for InterleaveShortest < I , J > where I : IndexedParallelIterator , J : IndexedParallelIterator < Item = I :: Item > , { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { self . interleave . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { self . interleave . with_producer (callback) } }
};
}
