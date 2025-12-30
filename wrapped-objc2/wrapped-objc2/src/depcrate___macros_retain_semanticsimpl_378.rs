// Generated macro for impl_378 (impl)
macro_rules! Depcrate___macros_retain_semanticsimpl_378 {
() => {
// Module: crate::__macros::retain_semantics
// Provides: {"impl_378"}
// Dependencies: {}
impl < T : Message > ConvertReturn < CopyFamily > for Option < Retained < T > > { type Inner = * mut T ; # [inline] unsafe fn convert_message_return (inner : Self :: Inner , _receiver_ptr : * mut AnyObject , _sel : Sel ,) -> Self { unsafe { Retained :: from_raw (inner) } } # [inline] fn convert_defined_return (self) -> Self :: Inner { Retained :: consume_as_ptr_option (self) } }
};
}
