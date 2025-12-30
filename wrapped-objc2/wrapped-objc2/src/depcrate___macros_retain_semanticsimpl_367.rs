// Generated macro for impl_367 (impl)
macro_rules! Depcrate___macros_retain_semanticsimpl_367 {
() => {
// Module: crate::__macros::retain_semantics
// Provides: {"impl_367"}
// Dependencies: {}
# [doc = " Convenience implementation for sending messages to `&Retained<T>`."] impl < 'a , T , Return , Kind , MethodFamily > RetainSemantics < & 'a Retained < T > , Return , Kind > for MethodFamily where T : Message , Kind : SendMessage , MethodFamily : RetainSemantics < * const T , Return , Kind > , { type ReceiverInner = * const T ; # [inline] fn prepare_message_send (receiver : & 'a Retained < T >) -> * const T { & * * receiver } # [inline] unsafe fn prepare_defined_method (_receiver : * const T) -> & 'a Retained < T > { unreachable ! () } type ReturnInner = < Self as RetainSemantics < * const T , Return , Kind > > :: ReturnInner ; # [inline] unsafe fn convert_message_return (ret : Self :: ReturnInner , receiver_ptr : * mut AnyObject , sel : Sel ,) -> Return { unsafe { < Self as RetainSemantics < * const T , Return , Kind > > :: convert_message_return (ret , receiver_ptr , sel ,) } } # [inline] fn convert_defined_return (ret : Return) -> Self :: ReturnInner { < Self as RetainSemantics < * const T , Return , Kind > > :: convert_defined_return (ret) } }
};
}
