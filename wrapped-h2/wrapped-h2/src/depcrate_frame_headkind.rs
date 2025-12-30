// Generated macro for Kind (enum)
macro_rules! Depcrate_frame_headKind {
() => {
// Module: crate::frame::head
// Provides: {"Kind"}
// Dependencies: {}
# [repr (u8)] # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub enum Kind { Data = 0 , Headers = 1 , Priority = 2 , Reset = 3 , Settings = 4 , PushPromise = 5 , Ping = 6 , GoAway = 7 , WindowUpdate = 8 , Continuation = 9 , Unknown , }
};
}
