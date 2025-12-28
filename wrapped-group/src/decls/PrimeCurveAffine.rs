macro_rules! deps {
    () => {
        GroupEncoding!();
        PrimeCurve!();
        Curve!();
    };
}

macro_rules! PrimeCurveAffine {
    () => {
        deps!();
        # [doc = " Affine representation of an elliptic curve point guaranteed to be"] # [doc = " in the correct prime order subgroup."] pub trait PrimeCurveAffine : GroupEncoding + Copy + Clone + Sized + Send + Sync + fmt :: Debug + PartialEq + Eq + 'static + Neg < Output = Self > + Mul < < Self as PrimeCurveAffine > :: Scalar , Output = < Self as PrimeCurveAffine > :: Curve > + for < 'r > Mul < & 'r < Self as PrimeCurveAffine > :: Scalar , Output = < Self as PrimeCurveAffine > :: Curve > { type Scalar : PrimeField ; type Curve : PrimeCurve < Affine = Self , Scalar = Self :: Scalar > ; # [doc = " Returns the additive identity."] fn identity () -> Self ; # [doc = " Returns a fixed generator of unknown exponent."] fn generator () -> Self ; # [doc = " Determines if this point represents the point at infinity; the"] # [doc = " additive identity."] fn is_identity (& self) -> Choice ; # [doc = " Converts this element to its curve representation."] fn to_curve (& self) -> Self :: Curve ; }
    };
}

PrimeCurveAffine!();