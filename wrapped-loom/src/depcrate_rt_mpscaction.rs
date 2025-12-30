// Generated macro for Action (enum)
macro_rules! Depcrate_rt_mpscAction {
() => {
// Module: crate::rt::mpsc
// Provides: {"Action"}
// Dependencies: {}
# [doc = " Actions performed on the Channel."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub (super) enum Action { # [doc = " Send a message"] MsgSend , # [doc = " Receive a message"] MsgRecv , }
};
}
