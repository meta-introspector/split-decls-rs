// Generated macro for impl_32 (impl)
macro_rules! Depcrate_nullableimpl_32 {
() => {
// Module: crate::nullable
// Provides: {"impl_32"}
// Dependencies: {}
impl From < NullableDateTime > for Option < jiff :: civil :: DateTime > { fn from (x : NullableDateTime) -> Self { x . 0 . map (Into :: into) } }
};
}
