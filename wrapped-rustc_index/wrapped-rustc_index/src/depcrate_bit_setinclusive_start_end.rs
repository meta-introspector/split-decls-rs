// Generated macro for inclusive_start_end (function)
macro_rules! Depcrate_bit_setinclusive_start_end {
() => {
// Module: crate::bit_set
// Provides: {"inclusive_start_end"}
// Dependencies: {}
# [inline] fn inclusive_start_end < T : Idx > (range : impl RangeBounds < T > , domain : usize ,) -> Option < (usize , usize) > { let start = match range . start_bound () . cloned () { Bound :: Included (start) => start . index () , Bound :: Excluded (start) => start . index () + 1 , Bound :: Unbounded => 0 , } ; let end = match range . end_bound () . cloned () { Bound :: Included (end) => end . index () , Bound :: Excluded (end) => end . index () . checked_sub (1) ? , Bound :: Unbounded => domain - 1 , } ; assert ! (end < domain) ; if start > end { return None ; } Some ((start , end)) }
};
}
