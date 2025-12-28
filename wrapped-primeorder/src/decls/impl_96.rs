macro_rules! deps {
    () => {
        ProjectivePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < 'a , C > Sum < & 'a ProjectivePoint < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { fn sum < I : Iterator < Item = & 'a ProjectivePoint < C > > > (iter : I) -> Self { iter . cloned () . sum () } }
    };
}

impl_96!()