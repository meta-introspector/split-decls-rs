// Generated macro for impl_39 (impl)
macro_rules! Depcrate_nullableimpl_39 {
() => {
// Module: crate::nullable
// Provides: {"impl_39"}
// Dependencies: {}
impl From < NullableDate > for Option < jiff :: civil :: Date > { fn from (x : NullableDate) -> Self { x . 0 . map (Into :: into) } }
};
}
