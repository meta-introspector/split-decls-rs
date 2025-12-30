// Generated macro for impl_1584 (impl)
macro_rules! Depcrate_sliceimpl_1584 {
() => {
// Module: crate::slice
// Provides: {"impl_1584"}
// Dependencies: {}
impl < 'data , T : 'data + Sync > Producer for IterProducer < 'data , T > { type Item = & 'data T ; type IntoIter = :: std :: slice :: Iter < 'data , T > ; fn into_iter (self) -> Self :: IntoIter { self . slice . iter () } fn split_at (self , index : usize) -> (Self , Self) { let (left , right) = self . slice . split_at (index) ; (IterProducer { slice : left } , IterProducer { slice : right }) } }
};
}
