// Generated macro for deconstruct_range (function)
macro_rules! Depcrate_codepointinvlist_utilsdeconstruct_range {
() => {
// Module: crate::codepointinvlist::utils
// Provides: {"deconstruct_range"}
// Dependencies: {}
# [doc = " Returns start (inclusive) and end (exclusive) bounds of [`RangeBounds`]"] pub fn deconstruct_range < T > (range : impl RangeBounds < T >) -> (u32 , u32) where T : Into < u32 > + Copy , { let from = match range . start_bound () { Included (b) => (* b) . into () , Excluded (_) => unreachable ! () , Unbounded => 0 , } ; let till = match range . end_bound () { Included (b) => (* b) . into () + 1 , Excluded (b) => (* b) . into () , Unbounded => (char :: MAX as u32) + 1 , } ; (from , till) }
};
}
