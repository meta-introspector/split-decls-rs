// Generated macro for impl_368 (impl)
macro_rules! Depcrate___macros_retain_semanticsimpl_368 {
() => {
// Module: crate::__macros::retain_semantics
// Provides: {"impl_368"}
// Dependencies: {}
# [doc = " Convenience implementation for sending messages to `ManuallyDrop<Retained<T>>`."] impl < T , Return , Kind , MethodFamily > RetainSemantics < ManuallyDrop < Retained < T > > , Return , Kind > for MethodFamily where T : Message , Kind : SendMessage , MethodFamily : RetainSemantics < * mut T , Return , Kind > , { type ReceiverInner = * mut T ; # [inline] fn prepare_message_send (receiver : ManuallyDrop < Retained < T > >) -> * mut T { Retained :: into_raw (ManuallyDrop :: into_inner (receiver)) } # [inline] unsafe fn prepare_defined_method (_receiver : * mut T) -> ManuallyDrop < Retained < T > > { unreachable ! () } type ReturnInner = < Self as RetainSemantics < * mut T , Return , Kind > > :: ReturnInner ; # [inline] unsafe fn convert_message_return (ret : Self :: ReturnInner , receiver_ptr : * mut AnyObject , sel : Sel ,) -> Return { unsafe { < Self as RetainSemantics < * mut T , Return , Kind > > :: convert_message_return (ret , receiver_ptr , sel ,) } } # [inline] fn convert_defined_return (ret : Return) -> Self :: ReturnInner { < Self as RetainSemantics < * mut T , Return , Kind > > :: convert_defined_return (ret) } }
};
}
