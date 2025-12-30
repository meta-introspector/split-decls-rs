// Generated macro for impl_333 (impl)
macro_rules! Depcrate___macros_msg_send_retainedimpl_333 {
() => {
// Module: crate::__macros::msg_send::retained
// Provides: {"impl_333"}
// Dependencies: {}
impl < Receiver , Return , MethodFamily > MsgSendSuper < Receiver , Return > for MethodFamily where MethodFamily : RetainSemantics < Receiver , Return , KindSendMessageSuper > , { type Inner = < < MethodFamily as RetainSemantics < Receiver , Return , KindSendMessageSuper > > :: ReceiverInner as MessageReceiver > :: __Inner ; # [inline] unsafe fn send_super_message < A : ConvertArguments > (receiver : Receiver , superclass : & AnyClass , sel : Sel , args : A ,) -> Return { let ptr = Self :: prepare_message_send (receiver) . __as_raw_receiver () ; let (args , _helper) = unsafe { A :: __into_arguments (args) } ; let ret = unsafe { MessageReceiver :: send_super_message (ptr , superclass , sel , args) } ; unsafe { Self :: convert_message_return (ret , ptr , sel) } } }
};
}
