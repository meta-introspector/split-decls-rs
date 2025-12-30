// Generated macro for PrimeCurveArithmetic (trait)
macro_rules! Depcrate_arithmeticPrimeCurveArithmetic {
() => {
// Module: crate::arithmetic
// Provides: {"PrimeCurveArithmetic"}
// Dependencies: {}
# [doc = " Prime order elliptic curve with projective arithmetic implementation."] pub trait PrimeCurveArithmetic : PrimeCurve + CurveArithmetic < ProjectivePoint = Self :: CurveGroup > { # [doc = " Prime order elliptic curve group."] type CurveGroup : group :: prime :: PrimeCurve < Affine = < Self as CurveArithmetic > :: AffinePoint > ; }
};
}
