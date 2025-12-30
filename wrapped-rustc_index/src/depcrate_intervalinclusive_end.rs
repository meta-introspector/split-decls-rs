// Generated macro for inclusive_end (function)
macro_rules! Depcrate_intervalinclusive_end {
() => {
// Module: crate::interval
// Provides: {"inclusive_end"}
// Dependencies: {}
# [inline] fn inclusive_end < T : Idx > (domain : usize , range : impl RangeBounds < T >) -> Option < u32 > { let end = match range . end_bound () { Bound :: Included (end) => end . index () as u32 , Bound :: Excluded (end) => end . index () . checked_sub (1) ? as u32 , Bound :: Unbounded => domain . checked_sub (1) ? as u32 , } ; Some (end) }
};
}
