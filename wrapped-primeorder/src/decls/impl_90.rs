macro_rules! deps {
    () => {
        ProjectivePoint!();
        AffinePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl < C > Add < AffinePoint < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { type Output = ProjectivePoint < C > ; fn add (self , other : AffinePoint < C >) -> ProjectivePoint < C > { ProjectivePoint :: add_mixed (& self , & other) } }
    };
}

impl_90!()