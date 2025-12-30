// Generated macro for impl_996 (impl)
macro_rules! Depcrate_base_matrix_viewimpl_996 {
() => {
// Module: crate::base::matrix_view
// Provides: {"impl_996"}
// Dependencies: {}
impl < D : Dim > DimRange < D > for Range < usize > { type Size = Dyn ; # [inline (always)] fn begin (& self , _ : D) -> usize { self . start } # [inline (always)] fn end (& self , _ : D) -> usize { self . end } # [inline (always)] fn size (& self , _ : D) -> Self :: Size { Dyn (self . end - self . start) } }
};
}
