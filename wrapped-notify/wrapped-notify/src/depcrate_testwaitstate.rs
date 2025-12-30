// Generated macro for WaitState (struct)
macro_rules! Depcrate_testWaitState {
() => {
// Module: crate::test
// Provides: {"WaitState"}
// Dependencies: {}
# [doc = " Result of a `wait` call on a [`Receiver`]"] # [derive (Debug)] pub struct WaitState { received : Vec < Event > , remain : Vec < Result < Event , Error > > , trackers : Trackers , }
};
}
