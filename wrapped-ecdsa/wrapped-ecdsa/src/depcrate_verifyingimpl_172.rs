// Generated macro for impl_172 (impl)
macro_rules! Depcrate_verifyingimpl_172 {
() => {
// Module: crate::verifying
// Provides: {"impl_172"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de , C > Deserialize < 'de > for VerifyingKey < C > where C : EcdsaCurve + AssociatedOid + CurveArithmetic + PointCompression , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , { fn deserialize < D > (deserializer : D) -> core :: result :: Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { PublicKey :: < C > :: deserialize (deserializer) . map (Into :: into) } }
};
}
