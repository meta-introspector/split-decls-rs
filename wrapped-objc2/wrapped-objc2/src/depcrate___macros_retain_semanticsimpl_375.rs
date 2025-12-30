// Generated macro for impl_375 (impl)
macro_rules! Depcrate___macros_retain_semanticsimpl_375 {
() => {
// Module: crate::__macros::retain_semantics
// Provides: {"impl_375"}
// Dependencies: {}
impl < T , Kind > RetainSemantics < PartialInit < T > , Option < Retained < T > > , Kind > for InitFamily where T : DefinedClass , Kind : IsSuper , { type ReceiverInner = * mut T ; # [inline] fn prepare_message_send (receiver : PartialInit < T >) -> * mut T { PartialInit :: into_ptr (receiver) } # [inline] unsafe fn prepare_defined_method (receiver : * mut T) -> PartialInit < T > { unsafe { PartialInit :: new (receiver) } } type ReturnInner = * mut T ; # [inline] unsafe fn convert_message_return (ret : * mut T , _receiver_ptr : * mut AnyObject , _sel : Sel ,) -> Option < Retained < T > > { let ret = unsafe { Retained :: from_raw (ret) } ; if let Some (ret) = ret { unsafe { set_finalized (ret . as_nonnull_ptr ()) } ; Some (ret) } else { None } } # [inline] fn convert_defined_return (ret : Option < Retained < T > >) -> * mut T { Retained :: consume_as_ptr_option (ret) } }
};
}
