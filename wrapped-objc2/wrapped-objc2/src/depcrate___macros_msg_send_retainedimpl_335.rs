// Generated macro for impl_335 (impl)
macro_rules! Depcrate___macros_msg_send_retainedimpl_335 {
() => {
// Module: crate::__macros::msg_send::retained
// Provides: {"impl_335"}
// Dependencies: {}
impl < Receiver , Return , MethodFamily > MsgSendError < Receiver , Retained < Return > > for MethodFamily where MethodFamily : MsgSend < Receiver , Option < Retained < Return > > > , { # [inline] unsafe fn send_message_error < A , E > (receiver : Receiver , sel : Sel , args : A ,) -> Result < Retained < Return > , Retained < E > > where * mut * mut E : Encode , A : TupleExtender < * mut * mut E > , < A as TupleExtender < * mut * mut E > > :: PlusOneArgument : ConvertArguments , E : ClassType , { let mut err : * mut E = ptr :: null_mut () ; let args = args . add_argument (& mut err) ; let ret = unsafe { Self :: send_message (receiver , sel , args) } ; if let Some (ret) = ret { Ok (ret) } else { Err (unsafe { encountered_error (err) }) } } }
};
}
