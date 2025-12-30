// Generated macro for impl_51 (impl)
macro_rules! Depcrate_nullableimpl_51 {
() => {
// Module: crate::nullable
// Provides: {"impl_51"}
// Dependencies: {}
impl ToDiesel for Option < jiff :: Span > { type Target = NullableSpan ; fn to_diesel (self) -> NullableSpan { NullableSpan (self . map (ToDiesel :: to_diesel)) } }
};
}
