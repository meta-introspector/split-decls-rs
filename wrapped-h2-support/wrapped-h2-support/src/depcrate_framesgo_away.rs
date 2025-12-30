// Generated macro for go_away (function)
macro_rules! Depcrate_framesgo_away {
() => {
// Module: crate::frames
// Provides: {"go_away"}
// Dependencies: {}
pub fn go_away < T > (id : T) -> Mock < frame :: GoAway > where T : Into < StreamId > , { Mock (frame :: GoAway :: new (id . into () , frame :: Reason :: NO_ERROR)) }
};
}
