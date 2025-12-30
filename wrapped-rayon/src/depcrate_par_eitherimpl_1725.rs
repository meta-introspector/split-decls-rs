// Generated macro for impl_1725 (impl)
macro_rules! Depcrate_par_eitherimpl_1725 {
() => {
// Module: crate::par_either
// Provides: {"impl_1725"}
// Dependencies: {}
impl < L , R > IndexedParallelIterator for Either < L , R > where L : IndexedParallelIterator , R : IndexedParallelIterator < Item = L :: Item > , { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { match self { Left (iter) => iter . drive (consumer) , Right (iter) => iter . drive (consumer) , } } fn len (& self) -> usize { self . as_ref () . either (L :: len , R :: len) } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { match self { Left (iter) => iter . with_producer (callback) , Right (iter) => iter . with_producer (callback) , } } }
};
}
