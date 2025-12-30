// Generated macro for impl_856 (impl)
macro_rules! Depcrate_iter_mapimpl_856 {
() => {
// Module: crate::iter::map
// Provides: {"impl_856"}
// Dependencies: {}
impl < I , F , R > ParallelIterator for Map < I , F > where I : ParallelIterator , F : Fn (I :: Item) -> R + Sync + Send , R : Send , { type Item = F :: Output ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = MapConsumer :: new (consumer , & self . map_op) ; self . base . drive_unindexed (consumer1) } fn opt_len (& self) -> Option < usize > { self . base . opt_len () } }
};
}
