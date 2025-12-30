// Generated macro for inclusive_start (function)
macro_rules! Depcrate_intervalinclusive_start {
() => {
// Module: crate::interval
// Provides: {"inclusive_start"}
// Dependencies: {}
# [inline] fn inclusive_start < T : Idx > (range : impl RangeBounds < T >) -> u32 { match range . start_bound () { Bound :: Included (start) => start . index () as u32 , Bound :: Excluded (start) => start . index () as u32 + 1 , Bound :: Unbounded => 0 , } }
};
}
