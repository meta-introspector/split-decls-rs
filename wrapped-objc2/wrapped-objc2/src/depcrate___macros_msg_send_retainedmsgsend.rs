// Generated macro for MsgSend (trait)
macro_rules! Depcrate___macros_msg_send_retainedMsgSend {
() => {
// Module: crate::__macros::msg_send::retained
// Provides: {"MsgSend"}
// Dependencies: {}
pub trait MsgSend < Receiver , Return > { # [track_caller] unsafe fn send_message < A : ConvertArguments > (receiver : Receiver , sel : Sel , args : A) -> Return ; }
};
}
