// Generated macro for indexed_range_impl (macro)
macro_rules! Depcrate_rangeindexed_range_impl {
() => {
// Module: crate::range
// Provides: {"indexed_range_impl"}
// Dependencies: {}
macro_rules ! indexed_range_impl { ($ t : ty) => { impl RangeInteger for $ t { private_impl ! { } fn drive_unindexed < C > (iter : Iter <$ t >, consumer : C) -> C :: Result where C : UnindexedConsumer <$ t >, { bridge (iter , consumer) } fn opt_len (iter : & Iter <$ t >) -> Option < usize > { Some (iter . range . len ()) } } impl IndexedRangeInteger for $ t { private_impl ! { } fn drive < C > (iter : Iter <$ t >, consumer : C) -> C :: Result where C : Consumer <$ t >, { bridge (iter , consumer) } fn len (iter : & Iter <$ t >) -> usize { iter . range . len () } fn with_producer < CB > (iter : Iter <$ t >, callback : CB) -> CB :: Output where CB : ProducerCallback <$ t >, { callback . callback (IterProducer { range : iter . range }) } } impl Producer for IterProducer <$ t > { type Item = < Range <$ t > as Iterator >:: Item ; type IntoIter = Range <$ t >; fn into_iter (self) -> Self :: IntoIter { self . range } fn split_at (self , index : usize) -> (Self , Self) { assert ! (index <= self . range . len ()) ; let mid = self . range . start . wrapping_add (index as $ t) ; let left = self . range . start .. mid ; let right = mid .. self . range . end ; (IterProducer { range : left } , IterProducer { range : right }) } } } ; }
};
}
