// Generated macro for impl_369 (impl)
macro_rules! Depcrate___macros_retain_semanticsimpl_369 {
() => {
// Module: crate::__macros::retain_semantics
// Provides: {"impl_369"}
// Dependencies: {}
impl < T : Message > ConvertReturn < NewFamily > for Option < Retained < T > > { type Inner = * mut T ; # [inline] unsafe fn convert_message_return (inner : Self :: Inner , _receiver_ptr : * mut AnyObject , _sel : Sel ,) -> Self { unsafe { Retained :: from_raw (inner) } } # [inline] fn convert_defined_return (self) -> Self :: Inner { Retained :: consume_as_ptr_option (self) } }
};
}
