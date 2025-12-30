// Generated macro for impl_385 (impl)
macro_rules! Depcrate___macros_retain_semanticsimpl_385 {
() => {
// Module: crate::__macros::retain_semantics
// Provides: {"impl_385"}
// Dependencies: {}
impl < T : Message > ConvertReturn < NoneFamily > for Retained < T > { type Inner = * mut T ; # [inline] unsafe fn convert_message_return (inner : Self :: Inner , receiver_ptr : * mut AnyObject , sel : Sel ,) -> Self { let ret = unsafe { Retained :: retain_autoreleased (inner) } ; if let Some (ret) = ret { ret } else { let receiver = unsafe { receiver_ptr . as_ref () } ; none_fail (receiver , sel) } } # [inline] fn convert_defined_return (self) -> Self :: Inner { Retained :: autorelease_return (self) } }
};
}
