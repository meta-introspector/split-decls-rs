// Generated macro for ping (function)
macro_rules! Depcrate_framesping {
() => {
// Module: crate::frames
// Provides: {"ping"}
// Dependencies: {}
pub fn ping (payload : [u8 ; 8]) -> Mock < frame :: Ping > { Mock (frame :: Ping :: new (payload)) }
};
}
