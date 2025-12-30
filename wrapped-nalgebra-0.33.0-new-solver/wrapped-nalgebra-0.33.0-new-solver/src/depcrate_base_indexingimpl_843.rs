// Generated macro for impl_843 (impl)
macro_rules! Depcrate_base_indexingimpl_843 {
() => {
// Module: crate::base::indexing
// Provides: {"impl_843"}
// Dependencies: {}
impl < D : Dim , T : Dim > DimRange < D > for ops :: RangeFrom < T > where D : DimSub < T > , { type Length = DimDiff < D , T > ; # [inline (always)] fn lower (& self , _ : D) -> usize { self . start . value () } # [inline (always)] fn length (& self , dimension : D) -> Self :: Length { dimension . sub (self . start) } # [inline (always)] fn contained_by (& self , _ : D) -> bool { true } }
};
}
