// Generated macro for impl_102 (impl)
macro_rules! Depcrate___macros_convertimpl_102 {
() => {
// Module: crate::__macros::convert
// Provides: {"impl_102"}
// Dependencies: {}
impl < MethodFamily > ConvertReturn < MethodFamily > for bool { type Inner = Bool ; # [inline] unsafe fn convert_message_return (inner : Self :: Inner , _receiver_ptr : * mut AnyObject , _sel : Sel ,) -> Self { inner . as_bool () } # [inline] fn convert_defined_return (self) -> Self :: Inner { Bool :: new (self) } }
};
}
