// Generated macro for impl_589 (impl)
macro_rules! Depcrate_types_nullableimpl_589 {
() => {
// Module: crate::types::nullable
// Provides: {"impl_589"}
// Dependencies: {}
impl < T : Clone > Nullable < & mut T > { # [doc = " Maps a `Nullable<&mut T>` to a `Nullable<T>` by cloning the contents of the nullable."] pub fn cloned (self) -> Nullable < T > { self . map (| t | t . clone ()) } }
};
}
