macro_rules! deps {
    () => {
        PrimeCurveParams!();
        ProjectivePoint!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < C > Add < & ProjectivePoint < C > > for & ProjectivePoint < C > where C : PrimeCurveParams , { type Output = ProjectivePoint < C > ; fn add (self , other : & ProjectivePoint < C >) -> ProjectivePoint < C > { ProjectivePoint :: add (self , other) } }
    };
}

impl_86!()