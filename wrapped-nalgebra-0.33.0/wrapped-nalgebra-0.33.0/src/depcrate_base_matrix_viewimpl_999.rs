// Generated macro for impl_999 (impl)
macro_rules! Depcrate_base_matrix_viewimpl_999 {
() => {
// Module: crate::base::matrix_view
// Provides: {"impl_999"}
// Dependencies: {}
impl < D : Dim > DimRange < D > for RangeFull { type Size = D ; # [inline (always)] fn begin (& self , _ : D) -> usize { 0 } # [inline (always)] fn end (& self , dim : D) -> usize { dim . value () } # [inline (always)] fn size (& self , dim : D) -> Self :: Size { dim } }
};
}
