// Generated macro for impl_1210 (impl)
macro_rules! Depcrate_iter_unzipimpl_1210 {
() => {
// Module: crate::iter::unzip
// Provides: {"impl_1210"}
// Dependencies: {}
impl < 'r , I , OP , CA > ParallelIterator for UnzipB < 'r , I , OP , CA > where I : ParallelIterator , OP : UnzipOp < I :: Item > , CA : UnindexedConsumer < OP :: Left > , { type Item = OP :: Right ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer = UnzipConsumer { op : & self . op , left : self . left_consumer , right : consumer , } ; let result = self . base . drive_unindexed (consumer) ; * self . left_result = Some (result . 0) ; result . 1 } fn opt_len (& self) -> Option < usize > { if OP :: indexable () { self . base . opt_len () } else { None } } }
};
}
