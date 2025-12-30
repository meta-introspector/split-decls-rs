// Generated macro for impl_818 (impl)
macro_rules! Depcrate_iter_intersperseimpl_818 {
() => {
// Module: crate::iter::intersperse
// Provides: {"impl_818"}
// Dependencies: {}
impl < I > ParallelIterator for Intersperse < I > where I : ParallelIterator < Item : Clone > , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < I :: Item > , { let consumer1 = IntersperseConsumer :: new (consumer , self . item) ; self . base . drive_unindexed (consumer1) } fn opt_len (& self) -> Option < usize > { match self . base . opt_len () ? { 0 => Some (0) , len => len . checked_add (len - 1) , } } }
};
}
