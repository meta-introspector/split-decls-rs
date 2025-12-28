macro_rules! deps {
    () => {
        CofactorCurve!();
        Curve!();
        GroupEncoding!();
    };
}

macro_rules! CofactorCurveAffine {
    () => {
        deps!();
        # [doc = " Affine representation of an elliptic curve point guaranteed to be"] # [doc = " in the correct prime order subgroup."] pub trait CofactorCurveAffine : GroupEncoding + Copy + Clone + Sized + Send + Sync + fmt :: Debug + PartialEq + Eq + 'static + Neg < Output = Self > + Mul < < Self as CofactorCurveAffine > :: Scalar , Output = < Self as CofactorCurveAffine > :: Curve > + for < 'r > Mul < & 'r < Self as CofactorCurveAffine > :: Scalar , Output = < Self as CofactorCurveAffine > :: Curve , > { type Scalar : PrimeField ; type Curve : CofactorCurve < Affine = Self , Scalar = Self :: Scalar > ; # [doc = " Returns the additive identity."] fn identity () -> Self ; # [doc = " Returns a fixed generator of unknown exponent."] fn generator () -> Self ; # [doc = " Determines if this point represents the point at infinity; the"] # [doc = " additive identity."] fn is_identity (& self) -> Choice ; # [doc = " Converts this element to its curve representation."] fn to_curve (& self) -> Self :: Curve ; }
    };
}

CofactorCurveAffine!()