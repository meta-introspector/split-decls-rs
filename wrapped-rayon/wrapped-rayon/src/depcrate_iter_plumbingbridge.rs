// Generated macro for bridge (function)
macro_rules! Depcrate_iter_plumbingbridge {
() => {
// Module: crate::iter::plumbing
// Provides: {"bridge"}
// Dependencies: {}
# [doc = " This helper function is used to \"connect\" a parallel iterator to a"] # [doc = " consumer. It will convert the `par_iter` into a producer P and"] # [doc = " then pull items from P and feed them to `consumer`, splitting and"] # [doc = " creating parallel threads as needed."] # [doc = ""] # [doc = " This is useful when you are implementing your own parallel"] # [doc = " iterators: it is often used as the definition of the"] # [doc = " [`drive_unindexed`] or [`drive`] methods."] # [doc = ""] # [doc = " [`drive_unindexed`]: super::ParallelIterator::drive_unindexed()"] # [doc = " [`drive`]: super::IndexedParallelIterator::drive()"] pub fn bridge < I , C > (par_iter : I , consumer : C) -> C :: Result where I : IndexedParallelIterator , C : Consumer < I :: Item > , { let len = par_iter . len () ; return par_iter . with_producer (Callback { len , consumer }) ; struct Callback < C > { len : usize , consumer : C , } impl < C , I > ProducerCallback < I > for Callback < C > where C : Consumer < I > , { type Output = C :: Result ; fn callback < P > (self , producer : P) -> C :: Result where P : Producer < Item = I > , { bridge_producer_consumer (self . len , producer , self . consumer) } } }
};
}
