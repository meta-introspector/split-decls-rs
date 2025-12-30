// Generated macro for impl_249 (impl)
macro_rules! Depcrate_matcher_support_zipped_iteratorimpl_249 {
() => {
// Module: crate::matcher_support::zipped_iterator
// Provides: {"impl_249"}
// Dependencies: {}
impl < I1 : Iterator , I2 : Iterator > Iterator for ZippedIterator < I1 , I2 > { type Item = (I1 :: Item , I2 :: Item) ; fn next (& mut self) -> Option < (I1 :: Item , I2 :: Item) > { match (self . left . next () , self . right . next ()) { (Some (v1) , Some (v2)) => { self . consumed_elements += 1 ; Some ((v1 , v2)) } (Some (_) , None) => { self . consumed_elements += 1 ; self . has_size_mismatch = true ; None } (None , Some (_)) => { self . has_size_mismatch = true ; None } (None , None) => None , } } }
};
}
