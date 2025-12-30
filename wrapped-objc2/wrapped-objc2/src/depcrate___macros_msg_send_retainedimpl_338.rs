// Generated macro for impl_338 (impl)
macro_rules! Depcrate___macros_msg_send_retainedimpl_338 {
() => {
// Module: crate::__macros::msg_send::retained
// Provides: {"impl_338"}
// Dependencies: {}
impl < Receiver , Return , MethodFamily > MsgSendSuperError < Receiver , Retained < Return > > for MethodFamily where MethodFamily : MsgSendSuper < Receiver , Option < Retained < Return > > > , { type Inner = < MethodFamily as MsgSendSuper < Receiver , Option < Retained < Return > > > > :: Inner ; # [inline] unsafe fn send_super_message_error < A , E > (receiver : Receiver , superclass : & AnyClass , sel : Sel , args : A ,) -> Result < Retained < Return > , Retained < E > > where * mut * mut E : Encode , A : TupleExtender < * mut * mut E > , < A as TupleExtender < * mut * mut E > > :: PlusOneArgument : ConvertArguments , E : ClassType , { let mut err : * mut E = ptr :: null_mut () ; let args = args . add_argument (& mut err) ; let ret = unsafe { Self :: send_super_message (receiver , superclass , sel , args) } ; if let Some (ret) = ret { Ok (ret) } else { Err (unsafe { encountered_error (err) }) } } }
};
}
