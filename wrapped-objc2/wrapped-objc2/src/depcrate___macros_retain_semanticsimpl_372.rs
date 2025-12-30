// Generated macro for impl_372 (impl)
macro_rules! Depcrate___macros_retain_semanticsimpl_372 {
() => {
// Module: crate::__macros::retain_semantics
// Provides: {"impl_372"}
// Dependencies: {}
impl < 'a , Return , Kind > RetainSemantics < & 'a AnyClass , Allocated < Return > , Kind > for AllocFamily where Return : Message , { type ReceiverInner = & 'a AnyClass ; # [inline] fn prepare_message_send (receiver : & 'a AnyClass) -> & 'a AnyClass { receiver } # [inline] unsafe fn prepare_defined_method (receiver : & 'a AnyClass) -> & 'a AnyClass { receiver } type ReturnInner = * mut Return ; # [inline] unsafe fn convert_message_return (ret : * mut Return , _receiver_ptr : * mut AnyObject , _sel : Sel ,) -> Allocated < Return > { unsafe { Allocated :: new (ret) } } # [inline] fn convert_defined_return (ret : Allocated < Return >) -> * mut Return { Allocated :: into_ptr (ret) } }
};
}
