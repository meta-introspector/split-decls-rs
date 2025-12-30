// Generated macro for MsgSendSuperError (trait)
macro_rules! Depcrate___macros_msg_send_retainedMsgSendSuperError {
() => {
// Module: crate::__macros::msg_send::retained
// Provides: {"MsgSendSuperError"}
// Dependencies: {}
pub trait MsgSendSuperError < Receiver , Return > { type Inner : ? Sized + RefEncode ; # [track_caller] unsafe fn send_super_message_error < A , E > (receiver : Receiver , superclass : & AnyClass , sel : Sel , args : A ,) -> Result < Return , Retained < E > > where * mut * mut E : Encode , A : TupleExtender < * mut * mut E > , < A as TupleExtender < * mut * mut E > > :: PlusOneArgument : ConvertArguments , E : ClassType ; # [track_caller] # [inline] unsafe fn send_super_message_static_error < A , E > (receiver : Receiver , sel : Sel , args : A ,) -> Result < Return , Retained < E > > where Self :: Inner : ClassType , < Self :: Inner as ClassType > :: Super : ClassType , * mut * mut E : Encode , A : TupleExtender < * mut * mut E > , < A as TupleExtender < * mut * mut E > > :: PlusOneArgument : ConvertArguments , E : ClassType , { unsafe { Self :: send_super_message_error (receiver , < Self :: Inner as ClassType > :: Super :: class () , sel , args ,) } } }
};
}
