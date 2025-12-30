// Generated macro for impl_849 (impl)
macro_rules! Depcrate_base_indexingimpl_849 {
() => {
// Module: crate::base::indexing
// Provides: {"impl_849"}
// Dependencies: {}
impl < D : Dim > DimRange < D > for ops :: RangeTo < usize > { type Length = Dyn ; # [inline (always)] fn lower (& self , _ : D) -> usize { 0 } # [inline (always)] fn length (& self , _ : D) -> Self :: Length { Dyn (self . end) } # [inline (always)] fn contained_by (& self , dimension : D) -> bool { self . end <= dimension . value () } }
};
}
