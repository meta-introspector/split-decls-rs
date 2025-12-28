macro_rules! deps {
    () => {
        PrimeCurveParams!();
        AffinePoint!();
        ProjectivePoint!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl < C > Add < & AffinePoint < C > > for & ProjectivePoint < C > where C : PrimeCurveParams , { type Output = ProjectivePoint < C > ; fn add (self , other : & AffinePoint < C >) -> ProjectivePoint < C > { ProjectivePoint :: add_mixed (self , other) } }
    };
}

impl_91!()