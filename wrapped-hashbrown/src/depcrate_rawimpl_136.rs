// Generated macro for impl_136 (impl)
macro_rules! Depcrate_rawimpl_136 {
() => {
// Module: crate::raw
// Provides: {"impl_136"}
// Dependencies: {}
impl < T > Iterator for RawIterHash < T > { type Item = Bucket < T > ; fn next (& mut self) -> Option < Bucket < T > > { unsafe { match self . inner . next () { Some (index) => { debug_assert ! (index <= self . inner . bucket_mask) ; let bucket = Bucket :: from_base_index (self . inner . ctrl . cast () , index) ; Some (bucket) } None => None , } } } }
};
}
