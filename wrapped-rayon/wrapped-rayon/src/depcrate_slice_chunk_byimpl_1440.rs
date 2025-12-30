// Generated macro for impl_1440 (impl)
macro_rules! Depcrate_slice_chunk_byimpl_1440 {
() => {
// Module: crate::slice::chunk_by
// Provides: {"impl_1440"}
// Dependencies: {}
impl < T , Slice , Pred > UnindexedProducer for ChunkByProducer < '_ , T , Slice , Pred > where Slice : ChunkBySlice < T > , Pred : Fn (& T , & T) -> bool + Send + Sync , { type Item = Slice ; fn split (self) -> (Self , Option < Self >) { if self . tail < 2 { return (Self { tail : 0 , .. self } , None) ; } let mid = self . tail / 2 ; let index = match self . slice . find (self . pred , mid , self . tail) { Some (i) => Some (mid + i) , None => self . slice . rfind (self . pred , mid + 1) , } ; if let Some (index) = index { let (left , right) = self . slice . split (index) ; let (left_tail , right_tail) = if index <= mid { (index , 0) } else { (mid + 1 , self . tail - index) } ; let left = Self { slice : left , tail : left_tail , .. self } ; let right = Self { slice : right , tail : right_tail , .. self } ; (left , Some (right)) } else { (Self { tail : 0 , .. self } , None) } } fn fold_with < F > (self , mut folder : F) -> F where F : Folder < Self :: Item > , { let Self { slice , pred , tail , .. } = self ; let (slice , tail) = if tail == slice . as_ref () . len () { (Some (slice) , None) } else if let Some (index) = slice . rfind (pred , tail) { let (left , right) = slice . split (index) ; (Some (left) , Some (right)) } else { (None , Some (slice)) } ; if let Some (slice) = slice { folder = folder . consume_iter (slice . chunk_by (pred)) ; } if let Some (tail) = tail { folder = folder . consume (tail) ; } folder } }
};
}
