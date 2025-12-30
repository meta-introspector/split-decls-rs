// Generated macro for diagonalize (function)
macro_rules! Depcrate_gutsdiagonalize {
() => {
// Module: crate::guts
// Provides: {"diagonalize"}
// Dependencies: {}
# [inline (always)] pub (crate) fn diagonalize < V : LaneWords4 > (mut x : State < V >) -> State < V > { x . a = x . a . shuffle_lane_words1230 () ; x . c = x . c . shuffle_lane_words3012 () ; x . d = x . d . shuffle_lane_words2301 () ; x }
};
}
