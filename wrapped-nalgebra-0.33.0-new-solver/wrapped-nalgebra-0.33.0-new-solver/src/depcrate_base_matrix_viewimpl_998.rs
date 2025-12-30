// Generated macro for impl_998 (impl)
macro_rules! Depcrate_base_matrix_viewimpl_998 {
() => {
// Module: crate::base::matrix_view
// Provides: {"impl_998"}
// Dependencies: {}
impl < D : Dim > DimRange < D > for RangeTo < usize > { type Size = Dyn ; # [inline (always)] fn begin (& self , _ : D) -> usize { 0 } # [inline (always)] fn end (& self , _ : D) -> usize { self . end } # [inline (always)] fn size (& self , _ : D) -> Self :: Size { Dyn (self . end) } }
};
}
