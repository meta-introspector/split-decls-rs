// Generated macro for impl_19 (impl)
macro_rules! Depcrate_rangeimpl_19 {
() => {
// Module: crate::range
// Provides: {"impl_19"}
// Dependencies: {}
impl RangeBounds for RangeTo < usize > { fn try_index (self , len : usize) -> Option < (usize , usize) > { if self . end <= len { Some ((0 , self . end)) } else { None } } }
};
}
