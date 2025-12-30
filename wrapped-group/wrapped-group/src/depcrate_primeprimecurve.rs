// Generated macro for PrimeCurve (trait)
macro_rules! Depcrate_primePrimeCurve {
() => {
// Module: crate::prime
// Provides: {"PrimeCurve"}
// Dependencies: {}
# [doc = " Efficient representation of an elliptic curve point guaranteed to be"] # [doc = " in the correct prime order subgroup."] pub trait PrimeCurve : Curve < AffineRepr = < Self as PrimeCurve > :: Affine > + PrimeGroup { type Affine : PrimeCurveAffine < Curve = Self , Scalar = Self :: Scalar > + Mul < Self :: Scalar , Output = Self > + for < 'r > Mul < & 'r Self :: Scalar , Output = Self > ; }
};
}
