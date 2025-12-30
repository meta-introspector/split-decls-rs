// Generated macro for impl_44 (impl)
macro_rules! Depcrate_nullableimpl_44 {
() => {
// Module: crate::nullable
// Provides: {"impl_44"}
// Dependencies: {}
impl ToDiesel for Option < jiff :: civil :: Time > { type Target = NullableTime ; fn to_diesel (self) -> NullableTime { NullableTime (self . map (ToDiesel :: to_diesel)) } }
};
}
