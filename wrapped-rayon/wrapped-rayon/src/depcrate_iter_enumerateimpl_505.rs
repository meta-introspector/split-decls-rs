// Generated macro for impl_505 (impl)
macro_rules! Depcrate_iter_enumerateimpl_505 {
() => {
// Module: crate::iter::enumerate
// Provides: {"impl_505"}
// Dependencies: {}
impl < I > ParallelIterator for Enumerate < I > where I : IndexedParallelIterator , { type Item = (usize , I :: Item) ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
