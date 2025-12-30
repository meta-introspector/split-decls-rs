// Generated macro for impl_1407 (impl)
macro_rules! Depcrate_range_inclusiveimpl_1407 {
() => {
// Module: crate::range_inclusive
// Provides: {"impl_1407"}
// Dependencies: {}
impl ParallelIterator for Iter < char > { type Item = char ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { convert_char ! (self . drive (consumer)) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
