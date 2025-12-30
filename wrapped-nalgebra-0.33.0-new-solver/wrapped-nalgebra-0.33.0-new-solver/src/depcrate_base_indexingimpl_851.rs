// Generated macro for impl_851 (impl)
macro_rules! Depcrate_base_indexingimpl_851 {
() => {
// Module: crate::base::indexing
// Provides: {"impl_851"}
// Dependencies: {}
impl < D : Dim > DimRange < D > for ops :: RangeToInclusive < usize > { type Length = Dyn ; # [inline (always)] fn lower (& self , _ : D) -> usize { 0 } # [inline (always)] fn length (& self , _ : D) -> Self :: Length { Dyn (self . end + 1) } # [inline (always)] fn contained_by (& self , dimension : D) -> bool { self . end < dimension . value () } }
};
}
