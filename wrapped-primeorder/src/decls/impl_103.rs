macro_rules! deps {
    () => {
        AffinePoint!();
        PrimeCurveParams!();
        ProjectivePoint!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < C > Sub < & AffinePoint < C > > for & ProjectivePoint < C > where C : PrimeCurveParams , { type Output = ProjectivePoint < C > ; fn sub (self , other : & AffinePoint < C >) -> ProjectivePoint < C > { ProjectivePoint :: sub_mixed (self , other) } }
    };
}

impl_103!();