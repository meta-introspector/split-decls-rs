macro_rules! deps {
    () => {
        ProjectivePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < C > Mul < & Scalar < C > > for & ProjectivePoint < C > where C : PrimeCurveParams , ProjectivePoint < C > : Double , { type Output = ProjectivePoint < C > ; fn mul (self , scalar : & Scalar < C >) -> ProjectivePoint < C > { ProjectivePoint :: mul (self , scalar) } }
    };
}

impl_109!();