// Generated macro for impl_339 (impl)
macro_rules! Depcrate___macros_msg_send_retainedimpl_339 {
() => {
// Module: crate::__macros::msg_send::retained
// Provides: {"impl_339"}
// Dependencies: {}
impl < Receiver , MethodFamily > MsgSendSuperError < Receiver , () > for MethodFamily where MethodFamily : MsgSendSuper < Receiver , bool > , { type Inner = < MethodFamily as MsgSendSuper < Receiver , bool > > :: Inner ; # [inline] unsafe fn send_super_message_error < A , E > (receiver : Receiver , superclass : & AnyClass , sel : Sel , args : A ,) -> Result < () , Retained < E > > where * mut * mut E : Encode , A : TupleExtender < * mut * mut E > , < A as TupleExtender < * mut * mut E > > :: PlusOneArgument : ConvertArguments , E : ClassType , { let mut err : * mut E = ptr :: null_mut () ; let args = args . add_argument (& mut err) ; let ret = unsafe { Self :: send_super_message (receiver , superclass , sel , args) } ; if ret { Ok (()) } else { Err (unsafe { encountered_error (err) }) } } }
};
}
