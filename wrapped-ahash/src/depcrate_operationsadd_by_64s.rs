// Generated macro for add_by_64s (function)
macro_rules! Depcrate_operationsadd_by_64s {
() => {
// Module: crate::operations
// Provides: {"add_by_64s"}
// Dependencies: {}
# [cfg (not (all (any (target_arch = "x86" , target_arch = "x86_64") , target_feature = "sse2" , not (miri))))] # [inline (always)] pub (crate) fn add_by_64s (a : [u64 ; 2] , b : [u64 ; 2]) -> [u64 ; 2] { [a [0] . wrapping_add (b [0]) , a [1] . wrapping_add (b [1])] }
};
}
