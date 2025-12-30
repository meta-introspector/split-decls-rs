// Generated macro for impl_997 (impl)
macro_rules! Depcrate_base_matrix_viewimpl_997 {
() => {
// Module: crate::base::matrix_view
// Provides: {"impl_997"}
// Dependencies: {}
impl < D : Dim > DimRange < D > for RangeFrom < usize > { type Size = Dyn ; # [inline (always)] fn begin (& self , _ : D) -> usize { self . start } # [inline (always)] fn end (& self , dim : D) -> usize { dim . value () } # [inline (always)] fn size (& self , dim : D) -> Self :: Size { Dyn (dim . value () - self . start) } }
};
}
