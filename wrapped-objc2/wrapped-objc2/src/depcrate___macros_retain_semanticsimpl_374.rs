// Generated macro for impl_374 (impl)
macro_rules! Depcrate___macros_retain_semanticsimpl_374 {
() => {
// Module: crate::__macros::retain_semantics
// Provides: {"impl_374"}
// Dependencies: {}
impl < T , Kind > RetainSemantics < Allocated < T > , Retained < T > , Kind > for InitFamily where T : Message , Kind : NotSuper , { type ReceiverInner = * mut T ; # [inline] fn prepare_message_send (receiver : Allocated < T >) -> * mut T { Allocated :: into_ptr (receiver) } # [inline] unsafe fn prepare_defined_method (receiver : * mut T) -> Allocated < T > { unsafe { Allocated :: new (receiver) } } type ReturnInner = * mut T ; # [inline] unsafe fn convert_message_return (ret : * mut T , receiver_ptr : * mut AnyObject , sel : Sel ,) -> Retained < T > { let ret = unsafe { Retained :: from_raw (ret) } ; if let Some (ret) = ret { ret } else { init_fail (receiver_ptr , sel) } } # [inline] fn convert_defined_return (ret : Retained < T >) -> * mut T { Retained :: into_raw (ret) } }
};
}
