// Generated macro for impl_837 (impl)
macro_rules! Depcrate_base_indexingimpl_837 {
() => {
// Module: crate::base::indexing
// Provides: {"impl_837"}
// Dependencies: {}
impl < D : Dim > DimRange < D > for usize { type Length = U1 ; # [inline (always)] fn lower (& self , _ : D) -> usize { * self } # [inline (always)] fn length (& self , _ : D) -> Self :: Length { Const :: < 1 > } # [inline (always)] fn contained_by (& self , dimension : D) -> bool { * self < dimension . value () } }
};
}
