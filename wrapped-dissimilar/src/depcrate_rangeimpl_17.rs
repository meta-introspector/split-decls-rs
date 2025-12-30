// Generated macro for impl_17 (impl)
macro_rules! Depcrate_rangeimpl_17 {
() => {
// Module: crate::range
// Provides: {"impl_17"}
// Dependencies: {}
impl RangeBounds for ops :: Range < usize > { fn try_index (self , len : usize) -> Option < (usize , usize) > { if self . start <= self . end && self . end <= len { Some ((self . start , self . end - self . start)) } else { None } } }
};
}
