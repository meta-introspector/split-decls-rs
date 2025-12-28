macro_rules! deps {
    () => {
        PrimeCurveParams!();
        AffinePoint!();
        ProjectivePoint!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < C , S > Mul < S > for & AffinePoint < C > where C : PrimeCurveParams , S : Borrow < Scalar < C > > , ProjectivePoint < C > : Double , { type Output = ProjectivePoint < C > ; # [inline] fn mul (self , scalar : S) -> ProjectivePoint < C > { ProjectivePoint :: < C > :: from (self) * scalar } }
    };
}

impl_44!();