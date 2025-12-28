macro_rules! deps {
    () => {
        PrimeCurveParams!();
        ProjectivePoint!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < C , S > Mul < S > for & ProjectivePoint < C > where Self : Double , C : PrimeCurveParams , S : Borrow < Scalar < C > > , { type Output = ProjectivePoint < C > ; fn mul (self , scalar : S) -> ProjectivePoint < C > { ProjectivePoint :: mul (self , scalar . borrow ()) } }
    };
}

impl_108!()