// Generated macro for impl_373 (impl)
macro_rules! Depcrate___macros_retain_semanticsimpl_373 {
() => {
// Module: crate::__macros::retain_semantics
// Provides: {"impl_373"}
// Dependencies: {}
impl < T , Kind > RetainSemantics < Allocated < T > , Option < Retained < T > > , Kind > for InitFamily where T : Message , Kind : NotSuper , { type ReceiverInner = * mut T ; # [inline] fn prepare_message_send (receiver : Allocated < T >) -> * mut T { Allocated :: into_ptr (receiver) } # [inline] unsafe fn prepare_defined_method (receiver : * mut T) -> Allocated < T > { unsafe { Allocated :: new (receiver) } } type ReturnInner = * mut T ; # [inline] unsafe fn convert_message_return (ret : * mut T , _receiver_ptr : * mut AnyObject , _sel : Sel ,) -> Option < Retained < T > > { unsafe { Retained :: from_raw (ret) } } # [inline] fn convert_defined_return (ret : Option < Retained < T > >) -> * mut T { Retained :: consume_as_ptr_option (ret) } }
};
}
