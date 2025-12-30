// Generated macro for MsgSendError (trait)
macro_rules! Depcrate___macros_msg_send_retainedMsgSendError {
() => {
// Module: crate::__macros::msg_send::retained
// Provides: {"MsgSendError"}
// Dependencies: {}
pub trait MsgSendError < Receiver , Return > { # [doc = " Add an extra error argument to the argument list, call `send_message`"] # [doc = " with that, and return an error if one occurred."] # [track_caller] unsafe fn send_message_error < A , E > (receiver : Receiver , sel : Sel , args : A ,) -> Result < Return , Retained < E > > where * mut * mut E : Encode , A : TupleExtender < * mut * mut E > , < A as TupleExtender < * mut * mut E > > :: PlusOneArgument : ConvertArguments , E : ClassType ; }
};
}
