// Generated macro for impl_37 (impl)
macro_rules! Depcrate_nullableimpl_37 {
() => {
// Module: crate::nullable
// Provides: {"impl_37"}
// Dependencies: {}
impl ToDiesel for Option < jiff :: civil :: Date > { type Target = NullableDate ; fn to_diesel (self) -> NullableDate { NullableDate (self . map (ToDiesel :: to_diesel)) } }
};
}
