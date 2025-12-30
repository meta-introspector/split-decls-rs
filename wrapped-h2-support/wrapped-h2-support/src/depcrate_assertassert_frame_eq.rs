// Generated macro for assert_frame_eq (function)
macro_rules! Depcrate_assertassert_frame_eq {
() => {
// Module: crate::assert
// Provides: {"assert_frame_eq"}
// Dependencies: {}
# [track_caller] pub fn assert_frame_eq < T : Into < Frame > , U : Into < Frame > > (t : T , u : U) { let actual : Frame = t . into () ; let expected : Frame = u . into () ; match (actual , expected) { (Frame :: Data (a) , Frame :: Data (b)) => { assert_eq ! (a . payload () . len () , b . payload () . len () , "assert_frame_eq data payload len") ; assert_eq ! (a , b , "assert_frame_eq") ; } (a , b) => { assert_eq ! (a , b , "assert_frame_eq") ; } } }
};
}
