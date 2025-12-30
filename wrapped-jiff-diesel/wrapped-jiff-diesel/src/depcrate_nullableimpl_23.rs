// Generated macro for impl_23 (impl)
macro_rules! Depcrate_nullableimpl_23 {
() => {
// Module: crate::nullable
// Provides: {"impl_23"}
// Dependencies: {}
impl From < NullableTimestamp > for Option < jiff :: Timestamp > { fn from (x : NullableTimestamp) -> Self { x . 0 . map (Into :: into) } }
};
}
