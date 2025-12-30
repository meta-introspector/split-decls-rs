// Generated macro for impl_1595 (impl)
macro_rules! Depcrate_sliceimpl_1595 {
() => {
// Module: crate::slice
// Provides: {"impl_1595"}
// Dependencies: {}
impl < 'data , T : 'data + Send > Producer for IterMutProducer < 'data , T > { type Item = & 'data mut T ; type IntoIter = :: std :: slice :: IterMut < 'data , T > ; fn into_iter (self) -> Self :: IntoIter { self . slice . iter_mut () } fn split_at (self , index : usize) -> (Self , Self) { let (left , right) = self . slice . split_at_mut (index) ; (IterMutProducer { slice : left } , IterMutProducer { slice : right } ,) } }
};
}
