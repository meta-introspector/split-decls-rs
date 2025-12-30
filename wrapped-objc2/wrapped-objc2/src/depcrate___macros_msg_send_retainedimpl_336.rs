// Generated macro for impl_336 (impl)
macro_rules! Depcrate___macros_msg_send_retainedimpl_336 {
() => {
// Module: crate::__macros::msg_send::retained
// Provides: {"impl_336"}
// Dependencies: {}
impl < Receiver , MethodFamily > MsgSendError < Receiver , () > for MethodFamily where MethodFamily : MsgSend < Receiver , bool > , { # [inline] unsafe fn send_message_error < A , E > (receiver : Receiver , sel : Sel , args : A ,) -> Result < () , Retained < E > > where * mut * mut E : Encode , A : TupleExtender < * mut * mut E > , < A as TupleExtender < * mut * mut E > > :: PlusOneArgument : ConvertArguments , E : ClassType , { let mut err : * mut E = ptr :: null_mut () ; let args = args . add_argument (& mut err) ; let ret = unsafe { Self :: send_message (receiver , sel , args) } ; if ret { Ok (()) } else { Err (unsafe { encountered_error (err) }) } } }
};
}
