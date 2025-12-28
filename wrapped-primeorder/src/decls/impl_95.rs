macro_rules! deps {
    () => {
        ProjectivePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl < C > Sum for ProjectivePoint < C > where C : PrimeCurveParams , { fn sum < I : Iterator < Item = Self > > (iter : I) -> Self { iter . fold (ProjectivePoint :: IDENTITY , | a , b | a + b) } }
    };
}

impl_95!()