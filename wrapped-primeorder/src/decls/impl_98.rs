macro_rules! deps {
    () => {
        PrimeCurveParams!();
        ProjectivePoint!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl < C > Sub < & ProjectivePoint < C > > for & ProjectivePoint < C > where C : PrimeCurveParams , { type Output = ProjectivePoint < C > ; fn sub (self , other : & ProjectivePoint < C >) -> ProjectivePoint < C > { ProjectivePoint :: sub (self , other) } }
    };
}

impl_98!();