// Generated macro for impl_588 (impl)
macro_rules! Depcrate_types_nullableimpl_588 {
() => {
// Module: crate::types::nullable
// Provides: {"impl_588"}
// Dependencies: {}
impl < T : Clone > Nullable < & T > { # [doc = " Maps a `Nullable<&T>` to a `Nullable<T>` by cloning the contents of the nullable."] pub fn cloned (self) -> Nullable < T > { self . map (| t | t . clone ()) } }
};
}
