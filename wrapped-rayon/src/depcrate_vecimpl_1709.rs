// Generated macro for impl_1709 (impl)
macro_rules! Depcrate_vecimpl_1709 {
() => {
// Module: crate::vec
// Provides: {"impl_1709"}
// Dependencies: {}
impl < 'data , T : 'data + Send > Producer for DrainProducer < 'data , T > { type Item = T ; type IntoIter = SliceDrain < 'data , T > ; fn into_iter (mut self) -> Self :: IntoIter { let slice = mem :: take (& mut self . slice) ; SliceDrain { iter : slice . iter_mut () , } } fn split_at (mut self , index : usize) -> (Self , Self) { let slice = mem :: take (& mut self . slice) ; let (left , right) = slice . split_at_mut (index) ; unsafe { (DrainProducer :: new (left) , DrainProducer :: new (right)) } } }
};
}
