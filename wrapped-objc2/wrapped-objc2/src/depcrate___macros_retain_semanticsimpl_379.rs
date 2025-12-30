// Generated macro for impl_379 (impl)
macro_rules! Depcrate___macros_retain_semanticsimpl_379 {
() => {
// Module: crate::__macros::retain_semantics
// Provides: {"impl_379"}
// Dependencies: {}
impl < T : Message > ConvertReturn < CopyFamily > for Retained < T > { type Inner = * mut T ; # [inline] unsafe fn convert_message_return (inner : Self :: Inner , _receiver_ptr : * mut AnyObject , _sel : Sel ,) -> Self { let ret = unsafe { Retained :: from_raw (inner) } ; if let Some (ret) = ret { ret } else { copy_fail () } } # [inline] fn convert_defined_return (self) -> Self :: Inner { Retained :: into_raw (self) } }
};
}
