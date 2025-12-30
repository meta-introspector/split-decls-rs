// Generated macro for indexed_range_impl (macro)
macro_rules! Depcrate_range_inclusiveindexed_range_impl {
() => {
// Module: crate::range_inclusive
// Provides: {"indexed_range_impl"}
// Dependencies: {}
macro_rules ! indexed_range_impl { ($ t : ty) => { parallel_range_impl ! { $ t } impl IndexedRangeInteger for $ t { private_impl ! { } fn drive < C > (iter : Iter <$ t >, consumer : C) -> C :: Result where C : Consumer <$ t >, { convert ! (iter . drive (consumer)) } fn len (iter : & Iter <$ t >) -> usize { iter . range . len () } fn with_producer < CB > (iter : Iter <$ t >, callback : CB) -> CB :: Output where CB : ProducerCallback <$ t >, { convert ! (iter . with_producer (callback)) } } } ; }
};
}
