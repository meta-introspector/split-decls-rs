// Generated macro for MsgSendSuper (trait)
macro_rules! Depcrate___macros_msg_send_retainedMsgSendSuper {
() => {
// Module: crate::__macros::msg_send::retained
// Provides: {"MsgSendSuper"}
// Dependencies: {}
pub trait MsgSendSuper < Receiver , Return > { type Inner : ? Sized + RefEncode ; # [track_caller] unsafe fn send_super_message < A : ConvertArguments > (receiver : Receiver , superclass : & AnyClass , sel : Sel , args : A ,) -> Return ; # [inline] # [track_caller] unsafe fn send_super_message_static < A : ConvertArguments > (receiver : Receiver , sel : Sel , args : A ,) -> Return where Self :: Inner : ClassType , < Self :: Inner as ClassType > :: Super : ClassType , { unsafe { Self :: send_super_message (receiver , < Self :: Inner as ClassType > :: Super :: class () , sel , args ,) } } }
};
}
