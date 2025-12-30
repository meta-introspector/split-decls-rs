// Generated macro for impl_586 (impl)
macro_rules! Depcrate_types_nullableimpl_586 {
() => {
// Module: crate::types::nullable
// Provides: {"impl_586"}
// Dependencies: {}
impl < T : Copy > Nullable < & T > { # [doc = " Maps a `Nullable<&T>` to a `Nullable<T>` by copying the contents of the nullable."] pub fn copied (self) -> Nullable < T > { self . map (| & t | t) } }
};
}
