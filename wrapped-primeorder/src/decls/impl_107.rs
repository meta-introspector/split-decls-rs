macro_rules! deps {
    () => {
        ProjectivePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl < C , S > Mul < S > for ProjectivePoint < C > where Self : Double , C : PrimeCurveParams , S : Borrow < Scalar < C > > , { type Output = Self ; fn mul (self , scalar : S) -> Self { ProjectivePoint :: mul (& self , scalar . borrow ()) } }
    };
}

impl_107!();