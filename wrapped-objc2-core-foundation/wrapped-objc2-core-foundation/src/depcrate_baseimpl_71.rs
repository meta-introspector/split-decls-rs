// Generated macro for impl_71 (impl)
macro_rules! Depcrate_baseimpl_71 {
() => {
// Module: crate::base
// Provides: {"impl_71"}
// Dependencies: {}
impl From < CFComparisonResult > for Ordering { # [inline] fn from (comparison_result : CFComparisonResult) -> Self { match comparison_result . 0 { ..= - 1 => Self :: Less , 0 => Self :: Equal , 1 .. => Self :: Greater , # [allow (unreachable_patterns)] _ => Self :: Equal , } } }
};
}
