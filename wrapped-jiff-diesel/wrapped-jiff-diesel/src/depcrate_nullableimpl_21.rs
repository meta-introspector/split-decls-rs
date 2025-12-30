// Generated macro for impl_21 (impl)
macro_rules! Depcrate_nullableimpl_21 {
() => {
// Module: crate::nullable
// Provides: {"impl_21"}
// Dependencies: {}
impl ToDiesel for Option < jiff :: Timestamp > { type Target = NullableTimestamp ; fn to_diesel (self) -> NullableTimestamp { NullableTimestamp (self . map (ToDiesel :: to_diesel)) } }
};
}
