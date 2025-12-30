// Generated macro for impl_22 (impl)
macro_rules! Depcrate_nullableimpl_22 {
() => {
// Module: crate::nullable
// Provides: {"impl_22"}
// Dependencies: {}
impl From < Option < jiff :: Timestamp > > for NullableTimestamp { fn from (x : Option < jiff :: Timestamp >) -> Self { Self (x . map (Into :: into)) } }
};
}
