// Generated macro for impl_873 (impl)
macro_rules! Depcrate_iter_map_withimpl_873 {
() => {
// Module: crate::iter::map_with
// Provides: {"impl_873"}
// Dependencies: {}
impl < I , T , F , R > ParallelIterator for MapWith < I , T , F > where I : ParallelIterator , T : Send + Clone , F : Fn (& mut T , I :: Item) -> R + Sync + Send , R : Send , { type Item = R ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = MapWithConsumer :: new (consumer , self . item , & self . map_op) ; self . base . drive_unindexed (consumer1) } fn opt_len (& self) -> Option < usize > { self . base . opt_len () } }
};
}
