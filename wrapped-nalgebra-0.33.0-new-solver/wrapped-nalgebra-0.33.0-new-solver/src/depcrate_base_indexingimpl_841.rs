// Generated macro for impl_841 (impl)
macro_rules! Depcrate_base_indexingimpl_841 {
() => {
// Module: crate::base::indexing
// Provides: {"impl_841"}
// Dependencies: {}
impl < D : Dim > DimRange < D > for ops :: RangeFrom < usize > { type Length = Dyn ; # [inline (always)] fn lower (& self , _ : D) -> usize { self . start } # [inline (always)] fn length (& self , dimension : D) -> Self :: Length { (self . start .. dimension . value ()) . length (dimension) } # [inline (always)] fn contained_by (& self , dimension : D) -> bool { self . start < dimension . value () } }
};
}
