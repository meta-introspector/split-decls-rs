// Generated macro for impl_384 (impl)
macro_rules! Depcrate___macros_retain_semanticsimpl_384 {
() => {
// Module: crate::__macros::retain_semantics
// Provides: {"impl_384"}
// Dependencies: {}
impl < T : Message > ConvertReturn < NoneFamily > for Option < Retained < T > > { type Inner = * mut T ; # [inline] unsafe fn convert_message_return (inner : Self :: Inner , _receiver_ptr : * mut AnyObject , _sel : Sel ,) -> Self { unsafe { Retained :: retain_autoreleased (inner) } } # [inline] fn convert_defined_return (self) -> Self :: Inner { Retained :: autorelease_return_option (self) } }
};
}
