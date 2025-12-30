// Generated macro for impl_566 (impl)
macro_rules! Depcrate_pat_utilimpl_566 {
() => {
// Module: crate::pat_util
// Provides: {"impl_566"}
// Dependencies: {}
impl < T : ExactSizeIterator > EnumerateAndAdjustIterator for T { fn enumerate_and_adjust (self , expected_len : usize , gap_pos : hir :: DotDotPos ,) -> EnumerateAndAdjust < Self > where Self : Sized , { let actual_len = self . len () ; EnumerateAndAdjust { enumerate : self . enumerate () , gap_pos : gap_pos . as_opt_usize () . unwrap_or (expected_len) , gap_len : expected_len - actual_len , } } }
};
}
