// Generated macro for impl_18 (impl)
macro_rules! Depcrate_rangeimpl_18 {
() => {
// Module: crate::range
// Provides: {"impl_18"}
// Dependencies: {}
impl RangeBounds for RangeFrom < usize > { fn try_index (self , len : usize) -> Option < (usize , usize) > { if self . start <= len { Some ((self . start , len - self . start)) } else { None } } }
};
}
