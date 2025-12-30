// Generated macro for impl_331 (impl)
macro_rules! Depcrate___macros_msg_send_retainedimpl_331 {
() => {
// Module: crate::__macros::msg_send::retained
// Provides: {"impl_331"}
// Dependencies: {}
impl < Receiver , Return , MethodFamily > MsgSend < Receiver , Return > for MethodFamily where MethodFamily : RetainSemantics < Receiver , Return , KindSendMessage > , { # [inline] unsafe fn send_message < A : ConvertArguments > (receiver : Receiver , sel : Sel , args : A) -> Return { let ptr = Self :: prepare_message_send (receiver) . __as_raw_receiver () ; let (args , _helper) = unsafe { A :: __into_arguments (args) } ; let ret = unsafe { MessageReceiver :: send_message (ptr , sel , args) } ; unsafe { Self :: convert_message_return (ret , ptr , sel) } } }
};
}
