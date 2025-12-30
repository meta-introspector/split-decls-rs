// Generated macro for impl_1372 (impl)
macro_rules! Depcrate_rangeimpl_1372 {
() => {
// Module: crate::range
// Provides: {"impl_1372"}
// Dependencies: {}
impl ParallelIterator for Iter < char > { type Item = char ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { convert_char ! (self . drive (consumer)) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
