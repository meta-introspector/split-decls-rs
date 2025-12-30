// Generated macro for impl_352 (impl)
macro_rules! Depcrate_merge_operatorimpl_352 {
() => {
// Module: crate::merge_operator
// Provides: {"impl_352"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a MergeOperands { type Item = & 'a [u8] ; type IntoIter = MergeOperandsIter < 'a > ; fn into_iter (self) -> Self :: IntoIter { Self :: IntoIter { operands : self , cursor : 0 , } } }
};
}
