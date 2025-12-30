// Generated macro for impl_100 (impl)
macro_rules! Depcrate___macros_convertimpl_100 {
() => {
// Module: crate::__macros::convert
// Provides: {"impl_100"}
// Dependencies: {}
impl < T : EncodeReturn , MethodFamily > ConvertReturn < MethodFamily > for T { type Inner = Self ; # [inline] unsafe fn convert_message_return (inner : Self :: Inner , _receiver_ptr : * mut AnyObject , _sel : Sel ,) -> Self { inner } # [inline] fn convert_defined_return (self) -> Self :: Inner { self } }
};
}
