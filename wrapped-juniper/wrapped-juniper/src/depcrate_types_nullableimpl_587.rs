// Generated macro for impl_587 (impl)
macro_rules! Depcrate_types_nullableimpl_587 {
() => {
// Module: crate::types::nullable
// Provides: {"impl_587"}
// Dependencies: {}
impl < T : Copy > Nullable < & mut T > { # [doc = " Maps a `Nullable<&mut T>` to a `Nullable<T>` by copying the contents of the nullable."] pub fn copied (self) -> Nullable < T > { self . map (| & mut t | t) } }
};
}
