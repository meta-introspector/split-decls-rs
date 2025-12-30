// Generated macro for impl_1000 (impl)
macro_rules! Depcrate_base_matrix_viewimpl_1000 {
() => {
// Module: crate::base::matrix_view
// Provides: {"impl_1000"}
// Dependencies: {}
impl < D : Dim > DimRange < D > for RangeInclusive < usize > { type Size = Dyn ; # [inline (always)] fn begin (& self , _ : D) -> usize { * self . start () } # [inline (always)] fn end (& self , _ : D) -> usize { * self . end () + 1 } # [inline (always)] fn size (& self , _ : D) -> Self :: Size { Dyn (* self . end () + 1 - * self . start ()) } }
};
}
