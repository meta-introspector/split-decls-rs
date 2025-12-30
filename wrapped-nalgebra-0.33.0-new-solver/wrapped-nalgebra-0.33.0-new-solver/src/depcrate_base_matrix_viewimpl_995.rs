// Generated macro for impl_995 (impl)
macro_rules! Depcrate_base_matrix_viewimpl_995 {
() => {
// Module: crate::base::matrix_view
// Provides: {"impl_995"}
// Dependencies: {}
impl < D : Dim > DimRange < D > for usize { type Size = U1 ; # [inline (always)] fn begin (& self , _ : D) -> usize { * self } # [inline (always)] fn end (& self , _ : D) -> usize { * self + 1 } # [inline (always)] fn size (& self , _ : D) -> Self :: Size { Const :: < 1 > } }
};
}
