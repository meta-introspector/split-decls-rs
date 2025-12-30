// Generated macro for impl_839 (impl)
macro_rules! Depcrate_base_indexingimpl_839 {
() => {
// Module: crate::base::indexing
// Provides: {"impl_839"}
// Dependencies: {}
impl < D : Dim > DimRange < D > for ops :: Range < usize > { type Length = Dyn ; # [inline (always)] fn lower (& self , _ : D) -> usize { self . start } # [inline (always)] fn length (& self , _ : D) -> Self :: Length { Dyn (self . end . saturating_sub (self . start)) } # [inline (always)] fn contained_by (& self , dimension : D) -> bool { (self . start < dimension . value ()) && (self . end <= dimension . value ()) } }
};
}
