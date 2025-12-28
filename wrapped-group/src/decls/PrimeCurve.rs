macro_rules! deps {
    () => {
        PrimeCurveAffine!();
        Curve!();
        PrimeGroup!();
    };
}

macro_rules! PrimeCurve {
    () => {
        deps!();
        # [doc = " Efficient representation of an elliptic curve point guaranteed to be"] # [doc = " in the correct prime order subgroup."] pub trait PrimeCurve : Curve < AffineRepr = < Self as PrimeCurve > :: Affine > + PrimeGroup { type Affine : PrimeCurveAffine < Curve = Self , Scalar = Self :: Scalar > + Mul < Self :: Scalar , Output = Self > + for < 'r > Mul < & 'r Self :: Scalar , Output = Self > ; }
    };
}

PrimeCurve!()