// Generated macro for impl_47 (impl)
macro_rules! Depcrate_affineimpl_47 {
() => {
// Module: crate::affine
// Provides: {"impl_47"}
// Dependencies: {}
impl < C > PrimeCurveAffine for AffinePoint < C > where C : PrimeCurveParams , CompressedPoint < C > : Send + Sync , FieldBytes < C > : Copy , FieldBytesSize < C > : ModulusSize , ProjectivePoint < C > : Double , CompressedPoint < C > : Copy , < UncompressedPointSize < C > as ArraySize > :: ArrayType < u8 > : Copy , { type Curve = ProjectivePoint < C > ; type Scalar = Scalar < C > ; fn identity () -> AffinePoint < C > { Self :: IDENTITY } fn generator () -> AffinePoint < C > { Self :: GENERATOR } fn is_identity (& self) -> Choice { self . is_identity () } fn to_curve (& self) -> ProjectivePoint < C > { ProjectivePoint :: from (* self) } }
};
}
