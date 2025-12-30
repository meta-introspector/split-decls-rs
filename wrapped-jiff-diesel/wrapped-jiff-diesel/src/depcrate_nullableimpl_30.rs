// Generated macro for impl_30 (impl)
macro_rules! Depcrate_nullableimpl_30 {
() => {
// Module: crate::nullable
// Provides: {"impl_30"}
// Dependencies: {}
impl ToDiesel for Option < jiff :: civil :: DateTime > { type Target = NullableDateTime ; fn to_diesel (self) -> NullableDateTime { NullableDateTime (self . map (ToDiesel :: to_diesel)) } }
};
}
