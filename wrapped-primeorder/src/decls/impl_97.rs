macro_rules! deps {
    () => {
        ProjectivePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl < C > Sub < ProjectivePoint < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { type Output = ProjectivePoint < C > ; fn sub (self , other : ProjectivePoint < C >) -> ProjectivePoint < C > { ProjectivePoint :: sub (& self , & other) } }
    };
}

impl_97!()