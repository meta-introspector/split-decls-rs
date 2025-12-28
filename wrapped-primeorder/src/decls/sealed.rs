macro_rules! deps {
    () => {
        PrimeCurveParams!();
        ProjectivePoint!();
        AffinePoint!();
    };
}

macro_rules! sealed {
    () => {
        deps!();
        mod sealed { use crate :: { AffinePoint , PrimeCurveParams , ProjectivePoint } ; # [doc = " Elliptic point arithmetic implementation"] # [doc = ""] # [doc = " Provides implementation of point arithmetic (point addition, point doubling) which"] # [doc = " might be optimized for the curve."] pub trait PointArithmetic < C : PrimeCurveParams > { # [doc = " Returns `lhs + rhs`"] fn add (lhs : & ProjectivePoint < C > , rhs : & ProjectivePoint < C >) -> ProjectivePoint < C > ; # [doc = " Returns `lhs + rhs`"] fn add_mixed (lhs : & ProjectivePoint < C > , rhs : & AffinePoint < C >) -> ProjectivePoint < C > ; # [doc = " Returns `point + point`"] fn double (point : & ProjectivePoint < C >) -> ProjectivePoint < C > ; } }
    };
}

sealed!()