// Generated macro for headers (function)
macro_rules! Depcrate_framesheaders {
() => {
// Module: crate::frames
// Provides: {"headers"}
// Dependencies: {}
pub fn headers < T > (id : T) -> Mock < frame :: Headers > where T : Into < StreamId > , { Mock (frame :: Headers :: new (id . into () , frame :: Pseudo :: default () , HeaderMap :: default () ,)) }
};
}
