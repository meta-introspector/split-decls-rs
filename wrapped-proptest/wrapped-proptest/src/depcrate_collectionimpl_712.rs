// Generated macro for impl_712 (impl)
macro_rules! Depcrate_collectionimpl_712 {
() => {
// Module: crate::collection
// Provides: {"impl_712"}
// Dependencies: {}
# [doc = " Adds `usize` to both start and end of the bounds."] # [doc = ""] # [doc = " Panics if adding to either end overflows `usize`."] impl Add < usize > for SizeRange { type Output = SizeRange ; fn add (self , rhs : usize) -> Self :: Output { let (start , end) = self . start_end_incl () ; size_range ((start + rhs) ..= (end + rhs)) } }
};
}
