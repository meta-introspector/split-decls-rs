// Generated macro for impl_376 (impl)
macro_rules! Depcrate___macros_retain_semanticsimpl_376 {
() => {
// Module: crate::__macros::retain_semantics
// Provides: {"impl_376"}
// Dependencies: {}
impl < T , Kind > RetainSemantics < PartialInit < T > , Retained < T > , Kind > for InitFamily where T : DefinedClass , Kind : IsSuper , { type ReceiverInner = * mut T ; # [inline] fn prepare_message_send (receiver : PartialInit < T >) -> * mut T { PartialInit :: into_ptr (receiver) } # [inline] unsafe fn prepare_defined_method (receiver : * mut T) -> PartialInit < T > { unsafe { PartialInit :: new (receiver) } } type ReturnInner = * mut T ; # [inline] unsafe fn convert_message_return (ret : * mut T , receiver_ptr : * mut AnyObject , sel : Sel ,) -> Retained < T > { let ret = unsafe { Retained :: from_raw (ret) } ; if let Some (ret) = ret { unsafe { set_finalized (ret . as_nonnull_ptr ()) } ; ret } else { init_fail (receiver_ptr , sel) } } # [inline] fn convert_defined_return (ret : Retained < T >) -> * mut T { Retained :: into_raw (ret) } }
};
}
