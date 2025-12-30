// Generated macro for impl_18 (impl)
macro_rules! Depcrate_split_producerimpl_18 {
() => {
// Module: crate::split_producer
// Provides: {"impl_18"}
// Dependencies: {}
impl < 'p , P , V , const INCL : bool > UnindexedProducer for SplitProducer < 'p , P , V , INCL > where V : Fissile < P > + Send , P : Sync , { type Item = V ; fn split (self) -> (Self , Option < Self >) { let mid = self . data . midpoint (self . tail) ; let index = match self . data . find (self . separator , mid , self . tail) { Some (i) => Some (mid + i) , None => self . data . rfind (self . separator , mid) , } ; if let Some (index) = index { let len = self . data . length () ; let (left , right) = self . data . split_once :: < INCL > (index) ; let (left_tail , right_tail) = if index < mid { (index , 0) } else { let right_index = len - right . length () ; (mid , self . tail - right_index) } ; let left = SplitProducer { data : left , tail : left_tail , .. self } ; let right = SplitProducer { data : right , tail : right_tail , .. self } ; (left , Some (right)) } else { (SplitProducer { tail : 0 , .. self } , None) } } fn fold_with < F > (self , folder : F) -> F where F : Folder < Self :: Item > , { self . fold_with (folder , false) } }
};
}
