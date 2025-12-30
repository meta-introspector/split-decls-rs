// Generated macro for reset (function)
macro_rules! Depcrate_framesreset {
() => {
// Module: crate::frames
// Provides: {"reset"}
// Dependencies: {}
pub fn reset < T > (id : T) -> Mock < frame :: Reset > where T : Into < StreamId > , { Mock (frame :: Reset :: new (id . into () , frame :: Reason :: NO_ERROR)) }
};
}
