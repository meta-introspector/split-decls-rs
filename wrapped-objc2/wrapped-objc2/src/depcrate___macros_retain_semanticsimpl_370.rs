// Generated macro for impl_370 (impl)
macro_rules! Depcrate___macros_retain_semanticsimpl_370 {
() => {
// Module: crate::__macros::retain_semantics
// Provides: {"impl_370"}
// Dependencies: {}
impl < T : Message > ConvertReturn < NewFamily > for Retained < T > { type Inner = * mut T ; # [inline] unsafe fn convert_message_return (inner : Self :: Inner , receiver_ptr : * mut AnyObject , sel : Sel ,) -> Self { let ret = unsafe { Retained :: from_raw (inner) } ; if let Some (ret) = ret { ret } else { let receiver = unsafe { receiver_ptr . as_ref () } ; new_fail (receiver , sel) } } # [inline] fn convert_defined_return (self) -> Self :: Inner { Retained :: into_raw (self) } }
};
}
