// Generated macro for impl_375 (impl)
macro_rules! Depcrate_iter_chainimpl_375 {
() => {
// Module: crate::iter::chain
// Provides: {"impl_375"}
// Dependencies: {}
impl < A , B > ParallelIterator for Chain < A , B > where A : ParallelIterator , B : ParallelIterator < Item = A :: Item > , { type Item = A :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let Chain { a , b } = self ; let (left , right , reducer) = if let Some (len) = a . opt_len () { consumer . split_at (len) } else { let reducer = consumer . to_reducer () ; (consumer . split_off_left () , consumer , reducer) } ; let (a , b) = join (| | a . drive_unindexed (left) , | | b . drive_unindexed (right)) ; reducer . reduce (a , b) } fn opt_len (& self) -> Option < usize > { self . a . opt_len () ? . checked_add (self . b . opt_len () ?) } }
};
}
