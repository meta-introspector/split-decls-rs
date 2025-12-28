macro_rules! deps {
    () => {
        AffinePoint!();
        ProjectivePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < C > From < & AffinePoint < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { fn from (p : & AffinePoint < C >) -> Self { Self :: from (* p) } }
    };
}

impl_61!()