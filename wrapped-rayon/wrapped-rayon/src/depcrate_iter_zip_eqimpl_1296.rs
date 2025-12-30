// Generated macro for impl_1296 (impl)
macro_rules! Depcrate_iter_zip_eqimpl_1296 {
() => {
// Module: crate::iter::zip_eq
// Provides: {"impl_1296"}
// Dependencies: {}
impl < A , B > IndexedParallelIterator for ZipEq < A , B > where A : IndexedParallelIterator , B : IndexedParallelIterator , { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self . zip , consumer) } fn len (& self) -> usize { self . zip . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { self . zip . with_producer (callback) } }
};
}
