// Generated macro for impl_83 (impl)
macro_rules! Depcrate_projectiveimpl_83 {
() => {
// Module: crate::projective
// Provides: {"impl_83"}
// Dependencies: {}
# [doc = " Prime order elliptic curves have a cofactor of 1."] impl < C > CofactorGroup for ProjectivePoint < C > where C : PrimeCurveParams , CompressedPoint < C > : Send + Sync , FieldBytes < C > : Copy , FieldBytesSize < C > : ModulusSize , CompressedPoint < C > : Copy , < UncompressedPointSize < C > as ArraySize > :: ArrayType < u8 > : Copy , { type Subgroup = ProjectivePoint < C > ; fn clear_cofactor (& self) -> Self :: Subgroup { * self } fn into_subgroup (self) -> CtOption < Self :: Subgroup > { CtOption :: new (self , Choice :: from (1)) } fn is_torsion_free (& self) -> Choice { Choice :: from (1) } }
};
}
