macro_rules! deps {
    () => {
        ProjectivePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl < C > Add < & ProjectivePoint < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { type Output = ProjectivePoint < C > ; fn add (self , other : & ProjectivePoint < C >) -> ProjectivePoint < C > { ProjectivePoint :: add (& self , other) } }
    };
}

impl_87!();