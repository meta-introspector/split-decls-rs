macro_rules! deps {
    () => {
        AffinePoint!();
        ProjectivePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < C > Sub < AffinePoint < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { type Output = ProjectivePoint < C > ; fn sub (self , other : AffinePoint < C >) -> ProjectivePoint < C > { ProjectivePoint :: sub_mixed (& self , & other) } }
    };
}

impl_102!()