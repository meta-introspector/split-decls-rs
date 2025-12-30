// Generated macro for impl_130 (impl)
macro_rules! Depcrate_projectiveimpl_130 {
() => {
// Module: crate::projective
// Provides: {"impl_130"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < C > Serialize for ProjectivePoint < C > where C : PrimeCurveParams , FieldBytesSize < C > : ModulusSize , CompressedPoint < C > : Copy , < UncompressedPointSize < C > as ArraySize > :: ArrayType < u8 > : Copy , { fn serialize < S > (& self , serializer : S) -> core :: result :: Result < S :: Ok , S :: Error > where S : ser :: Serializer , { self . to_affine () . serialize (serializer) } }
};
}
