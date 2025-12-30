// Generated macro for impl_845 (impl)
macro_rules! Depcrate_base_indexingimpl_845 {
() => {
// Module: crate::base::indexing
// Provides: {"impl_845"}
// Dependencies: {}
impl < D : Dim > DimRange < D > for ops :: RangeFull { type Length = D ; # [inline (always)] fn lower (& self , _ : D) -> usize { 0 } # [inline (always)] fn length (& self , dimension : D) -> Self :: Length { dimension } # [inline (always)] fn contained_by (& self , _ : D) -> bool { true } }
};
}
