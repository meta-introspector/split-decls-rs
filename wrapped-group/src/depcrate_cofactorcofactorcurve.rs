// Generated macro for CofactorCurve (trait)
macro_rules! Depcrate_cofactorCofactorCurve {
() => {
// Module: crate::cofactor
// Provides: {"CofactorCurve"}
// Dependencies: {}
# [doc = " Efficient representation of an elliptic curve point guaranteed to be"] # [doc = " in the correct prime order subgroup."] pub trait CofactorCurve : Curve < AffineRepr = < Self as CofactorCurve > :: Affine > + CofactorGroup { type Affine : CofactorCurveAffine < Curve = Self , Scalar = Self :: Scalar > + Mul < Self :: Scalar , Output = Self > + for < 'r > Mul < & 'r Self :: Scalar , Output = Self > ; }
};
}
