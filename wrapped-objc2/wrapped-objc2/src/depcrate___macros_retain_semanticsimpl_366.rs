// Generated macro for impl_366 (impl)
macro_rules! Depcrate___macros_retain_semanticsimpl_366 {
() => {
// Module: crate::__macros::retain_semantics
// Provides: {"impl_366"}
// Dependencies: {}
# [doc = " Generic implementation allowing message sending to normal encode return"] # [doc = " types."] impl < Receiver , Return , Kind , MethodFamily > RetainSemantics < Receiver , Return , Kind > for MethodFamily where Receiver : MessageReceiver , Return : ConvertReturn < MethodFamily > , { type ReceiverInner = Receiver ; # [inline] fn prepare_message_send (receiver : Receiver) -> Receiver { receiver } # [inline] unsafe fn prepare_defined_method (receiver : Receiver) -> Receiver { receiver } type ReturnInner = Return :: Inner ; # [inline] unsafe fn convert_message_return (ret : Return :: Inner , receiver_ptr : * mut AnyObject , sel : Sel ,) -> Return { unsafe { Return :: convert_message_return (ret , receiver_ptr , sel) } } # [inline] fn convert_defined_return (ret : Return) -> Return :: Inner { Return :: convert_defined_return (ret) } }
};
}
