// Generated macro for impl_847 (impl)
macro_rules! Depcrate_base_indexingimpl_847 {
() => {
// Module: crate::base::indexing
// Provides: {"impl_847"}
// Dependencies: {}
impl < D : Dim > DimRange < D > for ops :: RangeInclusive < usize > { type Length = Dyn ; # [inline (always)] fn lower (& self , _ : D) -> usize { * self . start () } # [inline (always)] fn length (& self , _ : D) -> Self :: Length { Dyn (if self . end () < self . start () { 0 } else { self . end () . wrapping_sub (self . start () . wrapping_sub (1)) }) } # [inline (always)] fn contained_by (& self , dimension : D) -> bool { (* self . start () < dimension . value ()) && (* self . end () < dimension . value ()) } }
};
}
