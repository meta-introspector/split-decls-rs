// Generated macro for impl_890 (impl)
macro_rules! Depcrate_iter_map_withimpl_890 {
() => {
// Module: crate::iter::map_with
// Provides: {"impl_890"}
// Dependencies: {}
impl < I , INIT , T , F , R > ParallelIterator for MapInit < I , INIT , F > where I : ParallelIterator , INIT : Fn () -> T + Sync + Send , F : Fn (& mut T , I :: Item) -> R + Sync + Send , R : Send , { type Item = R ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = MapInitConsumer :: new (consumer , & self . init , & self . map_op) ; self . base . drive_unindexed (consumer1) } fn opt_len (& self) -> Option < usize > { self . base . opt_len () } }
};
}
