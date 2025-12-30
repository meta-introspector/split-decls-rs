// Generated macro for impl_171 (impl)
macro_rules! Depcrate_verifyingimpl_171 {
() => {
// Module: crate::verifying
// Provides: {"impl_171"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < C > Serialize for VerifyingKey < C > where C : EcdsaCurve + AssociatedOid + CurveArithmetic + PointCompression , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , { fn serialize < S > (& self , serializer : S) -> core :: result :: Result < S :: Ok , S :: Error > where S : ser :: Serializer , { self . inner . serialize (serializer) } }
};
}
