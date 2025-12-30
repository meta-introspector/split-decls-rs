// Generated macro for impl_131 (impl)
macro_rules! Depcrate_projectiveimpl_131 {
() => {
// Module: crate::projective
// Provides: {"impl_131"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de , C > Deserialize < 'de > for ProjectivePoint < C > where C : PrimeCurveParams , FieldBytes < C > : Copy , FieldBytesSize < C > : ModulusSize , CompressedPoint < C > : Copy , { fn deserialize < D > (deserializer : D) -> core :: result :: Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { AffinePoint :: < C > :: deserialize (deserializer) . map (Self :: from) } }
};
}
