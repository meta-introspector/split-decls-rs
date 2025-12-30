// Generated macro for better_position (function)
macro_rules! Depcrate_iter_find_first_lastbetter_position {
() => {
// Module: crate::iter::find_first_last
// Provides: {"better_position"}
// Dependencies: {}
# [doc = " Returns true if pos1 is a better match than pos2 according to MatchPosition"] # [inline] fn better_position (pos1 : usize , pos2 : usize , mp : MatchPosition) -> bool { match mp { MatchPosition :: Leftmost => pos1 < pos2 , MatchPosition :: Rightmost => pos1 > pos2 , } }
};
}
